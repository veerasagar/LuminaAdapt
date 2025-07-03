import pandas as pd
import statsmodels.formula.api as smf

def load_and_prepare(csv_files):
    dfs = []
    for f in csv_files:
        df = pd.read_csv(f, parse_dates=['timestamp'])
        df['date'] = df['timestamp'].dt.date
        daily = df.groupby('date').agg(
            blue_avg=('blue_avg','mean'),
            usage_time=('active','sum')
        ).reset_index()
        dfs.append(daily)
    return pd.concat(dfs)


def fit_mixed_effects(data, psqi_scores):
    # psqi_scores: DataFrame with columns ['date','psqi']
    df = data.merge(psqi_scores, on='date')
    # model: psqi ~ blue_avg + usage_time + (1|participant)
    md = smf.mixedlm("psqi ~ blue_avg + usage_time", df, groups=df['participant'])
    mdf = md.fit()
    print(mdf.summary())
    return mdf