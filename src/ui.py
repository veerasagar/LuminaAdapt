import tkinter as tk
from tkinter import ttk
import matplotlib.pyplot as plt
import pandas as pd

class BlueLightGuardUI(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("BlueLightGuard")
        self.geometry("400x300")
        self.create_widgets()

    def create_widgets(self):
        ttk.Label(self, text="Sleep–Wake Window").pack(pady=5)
        self.start_var = tk.StringVar(value="22:00")
        self.end_var = tk.StringVar(value="06:00")
        ttk.Entry(self, textvariable=self.start_var).pack()
        ttk.Entry(self, textvariable=self.end_var).pack()
        ttk.Button(self, text="Save", command=self.save_window).pack(pady=10)
        ttk.Button(self, text="Show Last Week", command=self.show_plot).pack()

    def save_window(self):
        print(f"Window set: {self.start_var.get()} to {self.end_var.get()}")

    def show_plot(self):
        # Load combined data
        df = pd.read_csv('combined_data.csv', parse_dates=['date'])
        df['date'] = pd.to_datetime(df['date'])

        # Use the dataset’s max date so we always plot the most recent week
        max_date = df['date'].max()
        cutoff = max_date - pd.Timedelta(days=7)
        last7 = df[df['date'] >= cutoff]

        if last7.empty:
            print("[INFO] No data in the last 7 days to plot.")
            return

        # Plot
        plt.figure()
        plt.plot(last7['date'], last7['blue_avg'], marker='o', linestyle='-')
        plt.xlabel('Date')
        plt.ylabel('Average Blue Exposure')
        plt.title("Avg Blue Exposure (Last 7 Days in Data)")
        plt.gcf().autofmt_xdate()
        plt.grid(True)
        plt.tight_layout()
        plt.show()

if __name__ == '__main__':
    app = BlueLightGuardUI()
    app.mainloop()
