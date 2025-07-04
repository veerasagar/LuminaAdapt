import pandas as pd
from sklearn.ensemble import RandomForestRegressor

def train_sleep_model(data_csv='combined_data.csv'):
    df = pd.read_csv(data_csv, parse_dates=['date'])

    features = df[['blue_avg', 'usage_time']]
    target = df['sleep_latency']

    if len(df) < 5:
        print(f"[WARNING] Not enough samples for train/test split: only {len(df)} rows.")
        model = RandomForestRegressor(n_estimators=10)
        model.fit(features, target)
        print("Model trained on full dataset.")
        return model

    from sklearn.model_selection import train_test_split
    X_train, X_test, y_train, y_test = train_test_split(
        features, target, test_size=0.2, random_state=42
    )
    model = RandomForestRegressor(n_estimators=100)
    model.fit(X_train, y_train)
    print(f"Model R^2: {model.score(X_test, y_test):.2f}")
    return model

if __name__ == '__main__':
    train_sleep_model()
