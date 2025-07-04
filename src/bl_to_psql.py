from analysis import load_and_prepare, fit_mixed_effects

logs = ['../log/blue_light_log.csv']  # or list of all participant logs
data_df = load_and_prepare(logs)

# Pass the path to psqi_scores.csv explicitly
model = fit_mixed_effects(data_df, psqi_path='../log/psqi_scores.csv')
