use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use std::process::Command;
use actix_cors::Cors;

mod capture;
use capture::{start_screen_capture, FrameData};

// Re-using the structs and functions from your main application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NightLightConfig {
    temperature: u32,
    enabled: bool,
}

#[derive(Debug, Clone, Serialize)] // Added Clone trait
struct FrameAnalysis {
    average_brightness: f64,
    blue_intensity: f64,
    ambient_light_level: f64,
    timestamp: u64, // Unix timestamp in milliseconds
    frame_size: usize,
}

#[derive(Debug, Clone, Serialize)] // Added Clone trait
struct SystemStatus {
    running: bool,
    frames_processed: u64,
    current_analysis: Option<FrameAnalysis>,
    current_config: NightLightConfig,
    last_update: u64,
}

#[derive(Debug, Deserialize)]
struct UpdateConfigRequest {
    temperature: Option<u32>,
    enabled: Option<bool>,
}

// Global application state
struct AppState {
    status: Arc<Mutex<SystemStatus>>,
    config: Arc<Mutex<NightLightConfig>>,
    frame_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<FrameData>>>>,
}

impl Default for NightLightConfig {
    fn default() -> Self {
        Self {
            temperature: 4000,
            enabled: false,
        }
    }
}

// Copy the analysis functions from your main code
fn analyze_frame_for_nightlight(frame: &FrameData) -> FrameAnalysis {
    use pipewire::spa::param::video::VideoFormat;
    
    let brightness = calculate_average_brightness(frame);
    let blue_intensity = calculate_blue_intensity(frame);
    let ambient_level = estimate_ambient_light_level(frame);
    
    FrameAnalysis {
        average_brightness: brightness,
        blue_intensity,
        ambient_light_level: ambient_level,
        timestamp: frame.timestamp.elapsed().as_millis() as u64,
        frame_size: frame.data.len(),
    }
}

fn calculate_average_brightness(frame: &FrameData) -> f64 {
    use pipewire::spa::param::video::VideoFormat;
    
    if frame.data.is_empty() {
        return 0.0;
    }

    let total_pixels = (frame.width * frame.height) as usize;
    
    match frame.format {
        VideoFormat::RGB => {
            if frame.data.len() < total_pixels * 3 {
                return frame.data.iter().map(|&b| b as f64).sum::<f64>() / frame.data.len() as f64;
            }
            
            let sum: u64 = frame.data
                .chunks_exact(3)
                .take(total_pixels)
                .map(|rgb| {
                    let r = rgb[0] as u64;
                    let g = rgb[1] as u64;
                    let b = rgb[2] as u64;
                    (299 * r + 587 * g + 114 * b) / 1000
                })
                .sum();
            
            sum as f64 / total_pixels as f64
        },
        VideoFormat::RGBA => {
            if frame.data.len() < total_pixels * 4 {
                return frame.data.iter().map(|&b| b as f64).sum::<f64>() / frame.data.len() as f64;
            }
            
            let sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|rgba| {
                    let r = rgba[0] as u64;
                    let g = rgba[1] as u64;
                    let b = rgba[2] as u64;
                    (299 * r + 587 * g + 114 * b) / 1000
                })
                .sum();
            
            sum as f64 / total_pixels as f64
        },
        VideoFormat::RGBx => {
            if frame.data.len() < total_pixels * 4 {
                return frame.data.iter().map(|&b| b as f64).sum::<f64>() / frame.data.len() as f64;
            }
            
            let sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|rgbx| {
                    let r = rgbx[0] as u64;
                    let g = rgbx[1] as u64;
                    let b = rgbx[2] as u64;
                    (299 * r + 587 * g + 114 * b) / 1000
                })
                .sum();
            
            sum as f64 / total_pixels as f64
        },
        VideoFormat::BGRx => {
            if frame.data.len() < total_pixels * 4 {
                return frame.data.iter().map(|&b| b as f64).sum::<f64>() / frame.data.len() as f64;
            }
            
            let sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|bgrx| {
                    let b = bgrx[0] as u64;
                    let g = bgrx[1] as u64;
                    let r = bgrx[2] as u64;
                    (299 * r + 587 * g + 114 * b) / 1000
                })
                .sum();
            
            sum as f64 / total_pixels as f64
        },
        _ => {
            let sum: u64 = frame.data.iter().map(|&b| b as u64).sum();
            sum as f64 / frame.data.len() as f64
        }
    }
}

fn calculate_blue_intensity(frame: &FrameData) -> f64 {
    use pipewire::spa::param::video::VideoFormat;
    
    if frame.data.is_empty() {
        return 0.0;
    }

    let total_pixels = (frame.width * frame.height) as usize;
    
    match frame.format {
        VideoFormat::RGB => {
            if frame.data.len() < total_pixels * 3 {
                return 0.0;
            }
            
            let blue_sum: u64 = frame.data
                .chunks_exact(3)
                .take(total_pixels)
                .map(|rgb| rgb[2] as u64)
                .sum();
            
            blue_sum as f64 / total_pixels as f64
        },
        VideoFormat::RGBA => {
            if frame.data.len() < total_pixels * 4 {
                return 0.0;
            }
            
            let blue_sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|rgba| rgba[2] as u64)
                .sum();
            
            blue_sum as f64 / total_pixels as f64
        },
        VideoFormat::RGBx => {
            if frame.data.len() < total_pixels * 4 {
                return 0.0;
            }
            
            let blue_sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|rgbx| rgbx[2] as u64)
                .sum();
            
            blue_sum as f64 / total_pixels as f64
        },
        VideoFormat::BGRx => {
            if frame.data.len() < total_pixels * 4 {
                return 0.0;
            }
            
            let blue_sum: u64 = frame.data
                .chunks_exact(4)
                .take(total_pixels)
                .map(|bgrx| bgrx[0] as u64)
                .sum();
            
            blue_sum as f64 / total_pixels as f64
        },
        _ => 0.0,
    }
}

fn estimate_ambient_light_level(frame: &FrameData) -> f64 {
    use pipewire::spa::param::video::VideoFormat;
    
    let brightness = calculate_average_brightness(frame);
    let brightness_normalized = brightness / 255.0;
    
    if frame.data.is_empty() {
        return 0.5;
    }

    let total_pixels = (frame.width * frame.height) as usize;
    let mut variance_sum = 0.0;
    let mean_brightness = brightness;
    
    match frame.format {
        VideoFormat::RGB => {
            if frame.data.len() >= total_pixels * 3 {
                for rgb in frame.data.chunks_exact(3).take(total_pixels.min(1000)) {
                    let pixel_brightness = (299 * rgb[0] as u64 + 587 * rgb[1] as u64 + 114 * rgb[2] as u64) / 1000;
                    let diff = pixel_brightness as f64 - mean_brightness;
                    variance_sum += diff * diff;
                }
            }
        },
        VideoFormat::RGBA => {
            if frame.data.len() >= total_pixels * 4 {
                for rgba in frame.data.chunks_exact(4).take(total_pixels.min(1000)) {
                    let pixel_brightness = (299 * rgba[0] as u64 + 587 * rgba[1] as u64 + 114 * rgba[2] as u64) / 1000;
                    let diff = pixel_brightness as f64 - mean_brightness;
                    variance_sum += diff * diff;
                }
            }
        },
        VideoFormat::BGRx => {
            if frame.data.len() >= total_pixels * 4 {
                for bgrx in frame.data.chunks_exact(4).take(total_pixels.min(1000)) {
                    let pixel_brightness = (299 * bgrx[2] as u64 + 587 * bgrx[1] as u64 + 114 * bgrx[0] as u64) / 1000;
                    let diff = pixel_brightness as f64 - mean_brightness;
                    variance_sum += diff * diff;
                }
            }
        },
        _ => return brightness_normalized,
    }
    
    let variance = variance_sum / (1000.0_f64.min(total_pixels as f64));
    let contrast_factor = (variance / 10000.0).min(1.0);
    
    let ambient_estimate = (brightness_normalized * 0.7) + (contrast_factor * 0.3);
    ambient_estimate.min(1.0).max(0.0)
}

fn calculate_optimal_night_light_temperature(analysis: &FrameAnalysis) -> u32 {
    let brightness_factor = analysis.average_brightness / 255.0;
    let blue_factor = analysis.blue_intensity / 255.0;
    let ambient_factor = analysis.ambient_light_level;
    
    let base_temp_from_brightness = 3000.0 + (1.0 - brightness_factor) * 4500.0;
    let blue_adjustment = -(blue_factor * 1000.0);
    let ambient_adjustment = -(ambient_factor * 800.0);
    
    let calculated_temp = base_temp_from_brightness + blue_adjustment + ambient_adjustment;
    
    calculated_temp.max(3000.0).min(6500.0) as u32
}

fn set_night_light_temperature(temperature: u32) -> Result<(), std::io::Error> {
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.settings-daemon.plugins.color",
            "night-light-temperature",
            &temperature.to_string()
        ])
        .output()?;
    Ok(())
}

fn enable_night_light() -> Result<(), std::io::Error> {
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.settings-daemon.plugins.color",
            "night-light-enabled",
            "true"
        ])
        .output()?;

    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.settings-daemon.plugins.color", 
            "night-light-schedule-automatic",
            "false"
        ])
        .output()?;

    Ok(())
}

fn disable_night_light() -> Result<(), std::io::Error> {
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.settings-daemon.plugins.color",
            "night-light-enabled", 
            "false"
        ])
        .output()?;
    Ok(())
}

// API Handlers

async fn get_status(data: web::Data<AppState>) -> Result<HttpResponse> {
    let status = data.status.lock().unwrap().clone(); // Now works with Clone trait
    Ok(HttpResponse::Ok().json(status))
}

async fn get_config(data: web::Data<AppState>) -> Result<HttpResponse> {
    let config = data.config.lock().unwrap().clone();
    Ok(HttpResponse::Ok().json(config))
}

async fn update_config(
    data: web::Data<AppState>,
    req: web::Json<UpdateConfigRequest>
) -> Result<HttpResponse> {
    let mut config = data.config.lock().unwrap();
    let mut updated = false;

    if let Some(temperature) = req.temperature {
        if temperature >= 1000 && temperature <= 10000 {
            config.temperature = temperature;
            if config.enabled {
                if let Err(e) = set_night_light_temperature(temperature) {
                    return Ok(HttpResponse::InternalServerError().json(format!("Failed to set temperature: {}", e)));
                }
            }
            updated = true;
        } else {
            return Ok(HttpResponse::BadRequest().json("Temperature must be between 1000K and 10000K"));
        }
    }

    if let Some(enabled) = req.enabled {
        config.enabled = enabled;
        let result = if enabled {
            enable_night_light().and_then(|_| set_night_light_temperature(config.temperature))
        } else {
            disable_night_light()
        };

        if let Err(e) = result {
            return Ok(HttpResponse::InternalServerError().json(format!("Failed to change night light state: {}", e)));
        }
        updated = true;
    }

    if updated {
        Ok(HttpResponse::Ok().json(config.clone()))
    } else {
        Ok(HttpResponse::BadRequest().json("No valid parameters provided"))
    }
}

async fn start_monitoring(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut status = data.status.lock().unwrap();
    
    if status.running {
        return Ok(HttpResponse::BadRequest().json("Monitoring is already running"));
    }

    // Start screen capture
    match start_screen_capture().await {
        Ok(receiver) => {
            status.running = true;
            status.last_update = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            // Store the receiver in app state
            let mut frame_receiver = data.frame_receiver.lock().unwrap();
            *frame_receiver = Some(receiver);

            Ok(HttpResponse::Ok().json("Screen monitoring started"))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(format!("Failed to start monitoring: {}", e)))
        }
    }
}

async fn stop_monitoring(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut status = data.status.lock().unwrap();
    
    if !status.running {
        return Ok(HttpResponse::BadRequest().json("Monitoring is not running"));
    }

    status.running = false;
    
    // Remove the receiver
    let mut frame_receiver = data.frame_receiver.lock().unwrap();
    *frame_receiver = None;

    // Disable night light
    if let Err(e) = disable_night_light() {
        return Ok(HttpResponse::InternalServerError().json(format!("Failed to disable night light: {}", e)));
    }

    let mut config = data.config.lock().unwrap();
    config.enabled = false;

    Ok(HttpResponse::Ok().json("Screen monitoring stopped"))
}

async fn get_health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    })))
}

// Background task to process frames
async fn frame_processor(app_state: web::Data<AppState>) {
    let mut frame_count = 0u64;
    let mut last_adjustment = Instant::now();
    let adjustment_interval = Duration::from_secs(2);

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let receiver_opt = {
            let mut frame_receiver_guard = app_state.frame_receiver.lock().unwrap();
            if let Some(ref mut receiver) = *frame_receiver_guard {
                receiver.try_recv().ok()
            } else {
                None
            }
        };

        if let Some(frame) = receiver_opt {
            frame_count += 1;
            let analysis = analyze_frame_for_nightlight(&frame);

            // Update status
            {
                let mut status = app_state.status.lock().unwrap();
                status.frames_processed = frame_count;
                status.current_analysis = Some(analysis.clone()); // Now works with Clone trait
                status.last_update = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
            }

            // Auto-adjust temperature if enabled and enough time has passed
            if last_adjustment.elapsed() >= adjustment_interval {
                let config = app_state.config.lock().unwrap();
                if config.enabled {
                    let optimal_temperature = calculate_optimal_night_light_temperature(&analysis);
                    if optimal_temperature != config.temperature {
                        drop(config); // Release the lock before making system calls
                        if set_night_light_temperature(optimal_temperature).is_ok() {
                            let mut config = app_state.config.lock().unwrap();
                            config.temperature = optimal_temperature;
                        }
                    }
                }
                last_adjustment = Instant::now();
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize simple logging instead of env_logger
    println!("Starting Adaptive Night Light Web API...");

    let app_state = web::Data::new(AppState {
        status: Arc::new(Mutex::new(SystemStatus {
            running: false,
            frames_processed: 0,
            current_analysis: None,
            current_config: NightLightConfig::default(),
            last_update: 0,
        })),
        config: Arc::new(Mutex::new(NightLightConfig::default())),
        frame_receiver: Arc::new(Mutex::new(None)),
    });

    // Start background frame processor
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        frame_processor(app_state_clone).await;
    });

    

    println!("🌙 Adaptive Night Light Web API");
    println!("================================");
    println!("Server starting at http://localhost:8080");
    println!();
    println!("Available endpoints:");
    println!("  GET    /health         - Health check");
    println!("  GET    /status         - System status");
    println!("  GET    /config         - Current configuration");
    println!("  PUT    /config         - Update configuration");
    println!("  POST   /start          - Start monitoring");
    println!("  POST   /stop           - Stop monitoring");
    println!();

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .route("/health", web::get().to(get_health))
                    .route("/status", web::get().to(get_status))
                    .route("/config", web::get().to(get_config))
                    .route("/config", web::put().to(update_config))
                    .route("/start", web::post().to(start_monitoring))
                    .route("/stop", web::post().to(stop_monitoring))
            )
            .route("/health", web::get().to(get_health))
            .route("/status", web::get().to(get_status))
            .route("/config", web::get().to(get_config))
            .route("/config", web::put().to(update_config))
            .route("/start", web::post().to(start_monitoring))
            .route("/stop", web::post().to(stop_monitoring))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}