import tkinter as tk
import os

LOG_FILE = 'blue_light_log.csv'   # Path to your blue-light log
UPDATE_INTERVAL_MS = 1000         # milliseconds
MIN_OPACITY = 0.1
MAX_OPACITY = 0.7

def read_latest_blue_avg(log_path):
    """Quickly tail the last non-header line of the CSV and return blue_avg."""
    if not os.path.exists(log_path):
        return 0.0
    try:
        with open(log_path, 'rb') as f:
            # Jump near end (200 bytes should cover one line)
            f.seek(-200, os.SEEK_END)
            lines = f.readlines()
        # Find last non-empty, non-header line
        for raw in reversed(lines):
            line = raw.decode(errors='ignore').strip()
            if line and not line.startswith('timestamp'):
                parts = line.split(',')
                # CSV: timestamp,blue_avg,active,filter_level
                return float(parts[1])
    except Exception:
        pass
    return 0.0

def map_blue_to_opacity(blue):
    return MIN_OPACITY + (MAX_OPACITY - MIN_OPACITY) * blue

class ScreenOverlay(tk.Tk):
    def __init__(self):
        super().__init__()
        self.overrideredirect(True)
        self.attributes('-topmost', True)
        self.attributes('-alpha', MIN_OPACITY)
        w, h = self.winfo_screenwidth(), self.winfo_screenheight()
        self.geometry(f"{w}x{h}+0+0")

        self.canvas = tk.Canvas(self, highlightthickness=0)
        self.canvas.pack(fill='both', expand=True)
        self.canvas.create_rectangle(0, 0, w, h,
                                     fill='#FFFF66', outline='')

        # schedule first update
        self.after(UPDATE_INTERVAL_MS, self.update_opacity)

    def update_opacity(self):
        blue = read_latest_blue_avg(LOG_FILE)
        self.attributes('-alpha', map_blue_to_opacity(blue))
        # schedule next
        self.after(UPDATE_INTERVAL_MS, self.update_opacity)

if __name__ == '__main__':
    ScreenOverlay().mainloop()
