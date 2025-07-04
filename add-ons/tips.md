# Tips add-ons

Daemonizing: Use nohup python filter_control.py & or create a systemd user service so it always runs.

macOS helper: Until you implement setgamma, filter_control will error on Darwin. You can stub it or only run on Linux.

Extending the UI: Hook show_plot() to read and plot the past 7 days of real daily.blue_avg values.
