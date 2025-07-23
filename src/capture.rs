use std::os::fd::{IntoRawFd, OwnedFd};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use std::thread;

use ashpd::desktop::{
    screencast::{CursorMode, Screencast, SourceType, Stream as ScreencastStream},
    PersistMode,
};
use pipewire as pw;
use pw::{properties::properties, spa};

#[derive(Debug, Clone)]
pub struct FrameData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: pw::spa::param::video::VideoFormat,
    pub timestamp: Instant,
}

struct UserData {
    format: spa::param::video::VideoInfoRaw,
    last_frame_time: Arc<Mutex<Option<Instant>>>,
    frame_interval: Duration,
    frame_sender: mpsc::UnboundedSender<FrameData>,
    format_configured: Arc<Mutex<bool>>,
}

async fn open_portal() -> ashpd::Result<(ScreencastStream, OwnedFd)> {
    let proxy = Screencast::new().await?;
    let session = proxy.create_session().await?;
    proxy
        .select_sources(
            &session,
            CursorMode::Hidden,
            SourceType::Monitor.into(),
            false,
            None,
            PersistMode::DoNot,
        )
        .await?;

    let response = proxy.start(&session, None).await?.response()?;
    let stream = response
        .streams()
        .first()
        .expect("no stream found / selected")
        .to_owned();

    let fd = proxy.open_pipe_wire_remote(&session).await?;

    Ok((stream, fd))
}

pub async fn start_screen_capture() -> Result<mpsc::UnboundedReceiver<FrameData>, Box<dyn std::error::Error + Send + Sync>> {
    let (stream, fd) = open_portal().await?;
    let pipewire_node_id = stream.pipe_wire_node_id();

    let (frame_sender, frame_receiver) = mpsc::unbounded_channel();

    let sender_clone = frame_sender.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = start_streaming(pipewire_node_id, fd, sender_clone).await {
                eprintln!("Streaming error: {}", e);
            }
        });
    });

    Ok(frame_receiver)
}

async fn start_streaming(
    node_id: u32, 
    fd: OwnedFd, 
    frame_sender: mpsc::UnboundedSender<FrameData>
) -> Result<(), pw::Error> {
    pw::init();

    let mainloop = pw::main_loop::MainLoop::new(None)?;
    let context = pw::context::Context::new(&mainloop)?;
    let core = context.connect_fd(fd, None)?;

    let data = UserData {
        format: Default::default(),
        last_frame_time: Arc::new(Mutex::new(None)),
        frame_interval: Duration::from_millis(500),
        frame_sender,
        format_configured: Arc::new(Mutex::new(false)),
    };

    let stream = pw::stream::Stream::new(
        &core,
        "screen-capture",
        properties! {
            *pw::keys::MEDIA_TYPE => "Video",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Screen",
        },
    )?;

    let _listener = stream
        .add_local_listener_with_user_data(data)
        .state_changed(|_, _, old, new| {
            if matches!(new, pw::stream::StreamState::Error(_)) {
                eprintln!("Stream error state: {:?}", new);
            }
        })
        .param_changed(|_, user_data, id, param| {
            let Some(param) = param else {
                return;
            };
            
            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let (media_type, media_subtype) =
                match pw::spa::param::format_utils::parse_format(param) {
                    Ok(v) => v,
                    Err(_) => return,
                };

            if media_type != pw::spa::param::format::MediaType::Video
                || media_subtype != pw::spa::param::format::MediaSubtype::Raw
            {
                return;
            }

            if user_data.format.parse(param).is_ok() {
                *user_data.format_configured.lock().unwrap() = true;
            }
        })
        .process(|stream, user_data| {
            let format_configured = *user_data.format_configured.lock().unwrap();
            if !format_configured {
                return;
            }

            if let Some(mut buffer) = stream.dequeue_buffer() {
                let now = Instant::now();
                let should_process_frame = {
                    let mut last_time = user_data.last_frame_time.lock().unwrap();
                    match *last_time {
                        None => {
                            *last_time = Some(now);
                            true
                        },
                        Some(last) => {
                            if now.duration_since(last) >= user_data.frame_interval {
                                *last_time = Some(now);
                                true
                            } else {
                                false
                            }
                        }
                    }
                };

                if should_process_frame {
                    let datas = buffer.datas_mut();
                    
                    if !datas.is_empty() {
                        let data = &mut datas[0];
                        let chunk = data.chunk();
                        
                        if chunk.size() > 0 {
                            if let Some(data_slice) = data.data() {
                                let width = user_data.format.size().width;
                                let height = user_data.format.size().height;
                                let format = user_data.format.format();
                                
                                let bytes_per_pixel = match format {
                                    pw::spa::param::video::VideoFormat::RGB => 3,
                                    pw::spa::param::video::VideoFormat::RGBA => 4,
                                    pw::spa::param::video::VideoFormat::RGBx => 4,
                                    pw::spa::param::video::VideoFormat::BGRx => 4,
                                    pw::spa::param::video::VideoFormat::YUY2 => 2,
                                    pw::spa::param::video::VideoFormat::I420 => 1,
                                    _ => 4,
                                };
                                
                                let expected_size = (width * height * bytes_per_pixel) as usize;
                                let copy_len = std::cmp::min(data_slice.len(), expected_size);
                                let frame_data_vec = data_slice[..copy_len].to_vec();

                                let frame_data = FrameData {
                                    data: frame_data_vec,
                                    width,
                                    height,
                                    format,
                                    timestamp: now,
                                };

                                if user_data.frame_sender.send(frame_data).is_err() {
                                    return;
                                }
                                
                                std::thread::sleep(Duration::from_millis(1));
                            }
                        }
                    }
                }
            }
        })
        .register()?;

    let obj = pw::spa::pod::object!(
        pw::spa::utils::SpaTypes::ObjectParamFormat,
        pw::spa::param::ParamType::EnumFormat,
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::MediaType,
            Id,
            pw::spa::param::format::MediaType::Video
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::MediaSubtype,
            Id,
            pw::spa::param::format::MediaSubtype::Raw
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            pw::spa::param::video::VideoFormat::BGRx,
            pw::spa::param::video::VideoFormat::BGRx,
            pw::spa::param::video::VideoFormat::RGBx,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGBA,
            pw::spa::param::video::VideoFormat::YUY2,
            pw::spa::param::video::VideoFormat::I420,
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            pw::spa::utils::Rectangle {
                width: 1920,
                height: 1080
            },
            pw::spa::utils::Rectangle {
                width: 320,
                height: 240
            },
            pw::spa::utils::Rectangle {
                width: 4096,
                height: 4096
            }
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            pw::spa::utils::Fraction { num: 1, denom: 1 },
            pw::spa::utils::Fraction { num: 0, denom: 1 },
            pw::spa::utils::Fraction { num: 60, denom: 1 }
        ),
    );
    
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [spa::pod::Pod::from_bytes(&values).unwrap()];

    stream.connect(
        spa::utils::Direction::Input,
        Some(node_id),
        pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
        &mut params,
    )?;

    mainloop.run();

    Ok(())
}