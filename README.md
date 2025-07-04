# BlueLightGuard

## Execution

### 1. Start Data Collection & Filtering

* **What it does:** Samples your screen’s blue light, applies the filter, and logs every minute.
* **Script:**

  ```bash
  cd src
  python bluelightguard.py
  ```
  
* **Tip:** Leave this running during your entire study period.

### 2. (Optional) Verify Log Creation

* Ensure `blue_light_log.csv` appears in `src/` and is appending rows every minute.

### 3. Generate Dummy PSQI Scores

* **What it does:** Creates `psqi_scores.csv` in `data/` based on your log dates.
* **Script:**

  ```bash
  python generate_psqi_scores.py
  ```

* **Output:** `../data/psqi_scores.csv`

### 4. Combine Exposure Logs & PSQI into One CSV

* **What it does:** Merges `blue_light_log.csv` + `psqi_scores.csv`, adds a placeholder `sleep_latency`, and writes `combined_data.csv`.
* **Script:**

  ```bash
  python create_combined.py
  ```

* **Output:** `combined_data.csv` in `src/`

### 5. Run Statistical Analysis (Mixed-Effects Model)

* **What it does:** Loads `combined_data.csv`, fits the model `PSQI ~ blue_avg + usage_time`, and prints results.
* **Script:**

  ```bash
    python analysis.py
  ```

### 6. Train the ML Predictor

* **What it does:** Trains a RandomForest on `combined_data.csv` to predict `sleep_latency`, prints R².
* **Script:**

  ```bash
  cd src
  python ml_predict.py
  ```

### 7. Launch the GUI

* **What it does:** Opens a Tkinter window to set your sleep-wake window and plot the last week’s blue exposure.
* **Script:**

  ```bash
  cd src
  python ui.py
  ```

---

## Summary

1. **Collect & filter** (`bluelightguard.py`)
2. **Generate PSQI** (`generate_psqi_scores.py`)
3. **Merge & prepare** (`create_combined.py`)
4. **Analyze** (`analysis.py`)
5. **ML training** (`ml_predict.py`)
6. **Interact** (`ui.py`)
