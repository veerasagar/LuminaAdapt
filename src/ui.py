import tkinter as tk
from tkinter import ttk
from datetime import date

class BlueLightGuardUI(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("BlueLightGuard")
        self.geometry("400x300")
        self.create_widgets()

    def create_widgets(self):
        ttk.Label(self, text="Sleep–Wake Window").pack(pady=5)
        self.start_var = tk.StringVar(value="22:00")
        self.end_var   = tk.StringVar(value="06:00")
        ttk.Entry(self, textvariable=self.start_var).pack()
        ttk.Entry(self, textvariable=self.end_var).pack()

        ttk.Button(self, text="Save", command=self.save_window).pack(pady=10)
        ttk.Button(self, text="Show Last Week", command=self.show_plot).pack()

    def save_window(self):
        start = self.start_var.get()
        end   = self.end_var.get()
        print(f"Window set: {start} to {end}")

    def show_plot(self):
        # placeholder: launch a matplotlib plot window
        import matplotlib.pyplot as plt
        days = [date.today()]
        values = [0.5]
        plt.plot(days, values)
        plt.title("Avg Blue Exposure Last Week")
        plt.show()