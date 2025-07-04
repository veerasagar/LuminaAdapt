import ctypes
import time
import threading
import csv
from datetime import datetime
from PIL import ImageGrab
from pynput import keyboard, mouse
import numpy as np

# -------------------------------------------------------------------
# 1. Gamma‐ramp control (Windows)
# -------------------------------------------------------------------
gdi32 = ctypes.WinDLL('gdi32')
user32 = ctypes.WinDLL('user32')
  
def set_gamma(ramp):
    """Apply a 256‐entry gamma ramp (list of 3×256 shorts)."""
    hdc = user32.GetDC(0)
    # ramp must be WORD[3][256]
    ramp_array = (ctypes.c_ushort * (3*256))(*[int(v) for v in ramp])
    gdi32.SetDeviceGammaRamp(hdc, ctypes.byref(ramp_array))
    user32.ReleaseDC(0, hdc)

def make_gamma_ramp(temp_factor):
    """
    Build a simple ramp that reduces blue channel by temp_factor [0.0–1.0].
    Higher temp_factor → warmer (less blue).
    """
    ramp = []
    for channel in ('R','G','B'):
        for i in range(256):
            # identity ramp for R and G, scaled for B
            if channel == 'B':
                val = i * temp_factor
            else:
                val = i
            ramp.append(min(int(val), 255) * 256)  # WORD 0–65535
    return ramp

# -------------------------------------------------------------------
# 2. Screen‐sampling and logging
# -------------------------------------------------------------------
LOG_FILE = '../log/blue_light_log.csv'
SAMPLE_INTERVAL = 60  # seconds

# track user activity
active = False
last_activity = time.time()

def on_input(event):
    global active, last_activity
    active = True
    last_activity = time.time()

keyboard.Listener(on_press=on_input).start()
mouse.Listener(on_move=on_input, on_click=lambda *a: on_input(None),
               on_scroll=lambda *a: on_input(None)).start()

def sample_and_log():
    # Initialize CSV
    with open(LOG_FILE, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['timestamp','blue_avg','active','filter_level'])
    # Loop
    filter_level = 1.0  # no filter
    while True:
        # 1. grab small region and compute blue channel mean
        img = ImageGrab.grab(bbox=(0,0,100,100))  # top-left
        arr = np.array(img)
        blue_avg = float(arr[:,:,2].mean()) / 255.0
        
        # 2. detect if user is active in last minute
        is_active = (time.time() - last_activity) < SAMPLE_INTERVAL
        
        # 3. decide filter level
        # Example heuristic: if blue_avg > 0.3 and after 21:00, warm more
        hour = datetime.now().hour
        if hour >= 21 and is_active:
            target = 0.5  # reduce blue by 50%
        else:
            target = 1.0
        # smooth transition
        filter_level += (target - filter_level) * 0.1
        
        # 4. apply filter
        ramp = make_gamma_ramp(filter_level)
        set_gamma(ramp)
        
        # 5. log
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
