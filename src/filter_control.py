import platform
import subprocess
import ctypes

class FilterControl:
    def __init__(self):
        self.os = platform.system()

    def set_filter(self, level: float):
        """
        level: 0.0 (no blue reduction) to 1.0 (max reduction)
        """
        if self.os == 'Linux':
            self._linux_gamma(level)
        elif self.os == 'Darwin':
            self._macos_gamma(level)
        else:
            raise NotImplementedError(f"Unsupported OS: {self.os}")

    def _linux_gamma(self, level: float):
        # xrandr --output <DISPLAY> --gamma R:G:B --brightness <level>
        # we reduce blue by scaling B channel
        gamma = f"1:1:{1-level+0.1:.2f}"  # keep R,G=1, B reduced
        subprocess.run([
            'xrandr', '--output', 'eDP-1',  # adjust display name
            '--gamma', gamma,
            '--brightness', f"{1-level*0.5:.2f}"
        ], check=False)

    def _macos_gamma(self, level: float):
        # Placeholder: use CoreGraphics via ctypes or Swift bridge
        # Example uses a small Swift helper script compiled as 'setgamma'
        subprocess.run([
            'setgamma', f"{level:.2f}"  # hypothetical CLI
        ], check=False)