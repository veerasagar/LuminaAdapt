import pandas as pd
from analysis import load_and_prepare

if __name__ == '__main__':
    # List all log files per participant
    logs = ['blue_light_log.csv']  # add more file names if you have them
    data_df = load_and_prepare(logs)

    # Load PSQI scores from the data folder
    psqi_df = pd.read_csv('../data/psqi_scores.csv', parse_dates=['date'])

    # Normalize 'date' types for a clean merge
    data_df['date'] = pd.to_datetime(data_df['date']).dt.normalize()
    psqi_df['date'] = pd.to_datetime(psqi_df['date']).dt.normalize()

    # Ensure participant labels match (lowercase)
    data_df['participant'] = data_df['participant'].str.lower()
    psqi_df['participant'] = psqi_df['participant'].str.lower()

    # Merge on participant and date
    combined = data_df.merge(psqi_df, on=['participant', 'date'], how='inner')
    print(f"[INFO] Merged combined data rows: {combined.shape[0]}")
    if combined.empty:
        print("[ERROR] No matching participant/date. Check formats and values.")
        exit(1)

    # Add a placeholder sleep_latency column (replace with real data if available)
    combined['sleep_latency'] = combined['psqi'] * 4

    # Save combined data
    combined.to_csv('combined_data.csv', index=False)
    print(f"✅ Saved combined_data.csv with {len(combined)} rows.")
