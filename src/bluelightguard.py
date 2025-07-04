import ctypes
import time
import csv
from datetime import datetime
from PIL import ImageGrab
from pynput import keyboard, mouse
import numpy as np

# Gamma‐ramp control (Windows)
gdi32 = ctypes.WinDLL('gdi32')
user32 = ctypes.WinDLL('user32')

def set_gamma(ramp):
    """Apply a 256‐entry gamma ramp"""
    hdc = user32.GetDC(0)
    ramp_array = (ctypes.c_ushort * (3 * 256))(*ramp)
    gdi32.SetDeviceGammaRamp(hdc, ctypes.byref(ramp_array))
    user32.ReleaseDC(0, hdc)


def make_gamma_ramp(temp_factor):
    """
    Build a ramp that reduces blue channel by temp_factor [0.0–1.0].
    Higher temp_factor → warmer (less blue).
    """
    ramp = []
    for channel in ('R', 'G', 'B'):
        for i in range(256):
            val = i * temp_factor if channel == 'B' else i
            ramp.append(min(int(val), 255) * 256)
    return ramp

# Logging configuration
LOG_FILE = 'blue_light_log.csv'
SAMPLE_INTERVAL = 60  # seconds

active = False
last_activity = time.time()

def on_input(event):
    global active, last_activity
    active = True
    last_activity = time.time()

# Start input listeners
keyboard.Listener(on_press=on_input).start()
mouse.Listener(on_move=on_input,
               on_click=lambda *a: on_input(None),
               on_scroll=lambda *a: on_input(None)).start()

def sample_and_log():
    # Initialize CSV
    with open(LOG_FILE, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['timestamp', 'blue_avg', 'active', 'filter_level'])
    filter_level = 1.0
    while True:
        img = ImageGrab.grab(bbox=(0, 0, 100, 100))
        arr = np.array(img)
        blue_avg = float(arr[:, :, 2].mean()) / 255.0

        is_active = (time.time() - last_activity) < SAMPLE_INTERVAL
        hour = datetime.now().hour
        target = 0.5 if (hour >= 21 and is_active) else 1.0
        filter_level += (target - filter_level) * 0.1

        ramp = make_gamma_ramp(filter_level)
        set_gamma(ramp)

        with open(LOG_FILE, 'a', newline='') as f:
            writer = csv.writer(f)
            writer.writerow([
                datetime.now().isoformat(),
                f"{blue_avg:.3f}",
                int(is_active),
                f"{filter_level:.3f}"
            ])
        time.sleep(SAMPLE_INTERVAL)

if __name__ == '__main__':
    print("Starting BlueLightGuard logging… Press Ctrl+C to stop.")
    sample_and_log()