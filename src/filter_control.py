import platform
import ctypes
import subprocess

class FilterControl:
    def __init__(self):
        self.os = platform.system()

    def set_filter(self, level: float):
        """level: 0.0 (max blue reduction) to 1.0 (no reduction)"""
        if self.os == 'Linux':
            self._linux_gamma(level)
        elif self.os == 'Darwin':
            self._macos_gamma(level)
        elif self.os == 'Windows':
            self._windows_gamma(level)
        else:
            raise NotImplementedError(f"Unsupported OS: {self.os}")

    def _linux_gamma(self, level: float):
        gamma = f"1:1:{1 - level + 0.1:.2f}"
        subprocess.run([
            'xrandr', '--output', 'eDP-1', '--gamma', gamma,
            '--brightness', f"{1 - level * 0.5:.2f}"
        ], check=False)

    def _macos_gamma(self, level: float):
        subprocess.run(['setgamma', f"{level:.2f}"], check=False)

    def _windows_gamma(self, level: float):
        gdi32 = ctypes.WinDLL('gdi32')
        user32 = ctypes.WinDLL('user32')
        hdc = user32.GetDC(0)

        def make_ramp(level):
            ramp = []
            for channel in ('R', 'G', 'B'):
                for i in range(256):
                    val = i * (level if channel == 'B' else 1)
                    ramp.append(min(int(val), 255) * 256)
            return (ctypes.c_ushort * (3 * 256))(*ramp)

        ramp = make_ramp(level)
        gdi32.SetDeviceGammaRamp(hdc, ctypes.byref(ramp))
        user32.ReleaseDC(0, hdc)
