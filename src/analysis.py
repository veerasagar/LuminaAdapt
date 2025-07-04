import pandas as pd
import statsmodels.formula.api as smf
import os

def load_and_prepare(csv_files):
    dfs = []
    for f in csv_files:
        df = pd.read_csv(f, parse_dates=['timestamp'])
        participant = os.path.splitext(os.path.basename(f))[0]
        df['participant'] = participant
        df['date'] = df['timestamp'].dt.date
        daily = df.groupby(['participant','date']).agg(
            blue_avg=('blue_avg','mean'),
            usage_time=('active','sum')
        ).reset_index()
        dfs.append(daily)
    return pd.concat(dfs, ignore_index=True)


def fit_mixed_effects(data, psqi_path='../data/psqi_scores.csv'):
    psqi_df = pd.read_csv(psqi_path, parse_dates=['date'])

    # Normalize types
    data['date'] = pd.to_datetime(data['date']).dt.normalize()
    psqi_df['date'] = pd.to_datetime(psqi_df['date']).dt.normalize()
    data['participant'] = data['participant'].str.lower()
    psqi_df['participant'] = psqi_df['participant'].str.lower()

    print(f"[INFO] Ready to merge: data rows={len(data)}, psqi rows={len(psqi_df)}")
    df = data.merge(psqi_df, on=['participant','date'])
    print(f"[INFO] Merged rows: {df.shape[0]}")
    if df.empty:
        print("[ERROR] No matching records. Check participant/date keys.")
        return None

    md = smf.mixedlm("psqi ~ blue_avg + usage_time", df, groups=df['participant'])
    mdf = md.fit()
    print(mdf.summary())
    return mdf