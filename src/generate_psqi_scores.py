import pandas as pd
import random
import os

def generate_psqi_scores(csv_files, output_file='../data/psqi_scores.csv'):
    entries = []
    for f in csv_files:
        df = pd.read_csv(f, parse_dates=['timestamp'])
        participant = os.path.splitext(os.path.basename(f))[0]
        df['date'] = df['timestamp'].dt.date
        for d in df['date'].unique():
            entries.append({
                'participant': participant,
                'date': d,
                'psqi': random.randint(5,12)
            })
    psqi_df = pd.DataFrame(entries)
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    psqi_df.to_csv(output_file, index=False)
    print(f"Generated '{output_file}' ({len(psqi_df)} rows)")

if __name__ == '__main__':
    generate_psqi_scores(['blue_light_log.csv'])