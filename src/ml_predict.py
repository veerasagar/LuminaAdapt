import pandas as pd
from sklearn.ensemble import RandomForestRegressor
from sklearn.model_selection import train_test_split


def train_sleep_model(data):
    # data: DataFrame with features and target 'sleep_latency'
    features = data[['blue_avg','usage_time']]
    target   = data['sleep_latency']
    X_train, X_test, y_train, y_test = train_test_split(features, target, test_size=0.2)
    model = RandomForestRegressor(n_estimators=100)
    model.fit(X_train, y_train)
    score = model.score(X_test, y_test)
    print(f"Model R^2: {score:.2f}")
    return model


# ---------- Example Usage ----------
if __name__ == '__main__':
    # Example: filter control
    fc = FilterControl()
    fc.set_filter(0.5)  # reduce blue 50%

    # Launch UI
    app = BlueLightGuardUI()
    app.mainloop()
