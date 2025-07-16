# metrics

1. **Mean Absolute Error (MAE)**

   $$
     \text{MAE} = \frac{1}{N}\sum_{i=1}^N \bigl|\hat{y}_i - y_i\bigr|
   $$

   * **Interpretation:** On average, the model’s predicted sleep‑latency (in minutes) is off by this amount.
   * **Units:** Minutes.
   * **Robustness:** Less sensitive to large outliers than RMSE.

2. **Root Mean Squared Error (RMSE)**

   $$
     \text{RMSE} = \sqrt{\frac{1}{N}\sum_{i=1}^N (\hat{y}_i - y_i)^2}
   $$

   * **Interpretation:** It’s the standard deviation of the residuals (prediction errors).
   * **Units:** Minutes.
   * **Sensitivity:** Punishes larger errors more heavily (because of the square), so it highlights when the model occasionally makes big mistakes.

3. **Coefficient of Determination (R²)**

   $$
     R^2 = 1 - \frac{\sum_{i}(y_i - \hat{y}_i)^2}{\sum_{i}(y_i - \bar{y})^2}
   $$

   * **Interpretation:** The fraction of variance in the true sleep‑latency that your model explains.
   * **Scale:**

     * $R^2 = 1$ means perfect predictions.
     * $R^2 = 0$ means the model is no better than always predicting the mean.
     * Negative $R^2$ means it’s worse than the mean predictor.

---

* **Low MAE & RMSE** means your predicted sleep‑latency is close to the actual values (in minutes).
* **High R²** (e.g. 0.7–0.9) indicates the model captures most of the night‑to‑night variability; **low** or **negative** R² suggests it’s missing key patterns.

By examining these three metrics together, you get a balanced view of average error magnitude (MAE), sensitivity to large mistakes (RMSE), and overall explanatory power (R²).
