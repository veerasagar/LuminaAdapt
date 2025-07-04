import pandas as pd
from analysis import load_and_prepare, fit_mixed_effects

# load exposure logs
exposure_df = load_and_prepare(['blue_light_log.csv'])
# load PSQI scores
psqi_df = pd.read_csv('psqi_scores.csv', parse_dates=['date'])
# fit the mixed‐effects model
model = fit_mixed_effects(exposure_df, psqi_df)
