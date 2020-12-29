use crate::summary::mean;

/// Calculates the autocovariance of lag k of a vector of time series data,
/// assuming that the points are equally spaced in time.
pub fn acovf(ts: &[f64], k: usize) -> f64 {
    let n = ts.len();
    let ts_mean = mean(&ts);
    1. / n as f64
        * (k..n)
            .into_iter()
            .map(|i| (ts[i] - ts_mean) * (ts[i - k] - ts_mean))
            .sum::<f64>()
}

/// Calculates the autocorrelation of lag k of a vector of time series data,
/// assuming that the points are equally spaced in time.
pub fn acf(ts: &[f64], k: usize) -> f64 {
    let n = ts.len();
    let ts_mean = mean(&ts);
    let numerator: f64 = 1. / n as f64
        * (k..n)
            .into_iter()
            .map(|i| (ts[i] - ts_mean) * (ts[i - k] - ts_mean))
            .sum::<f64>();
    let denominator: f64 = (0..n)
        .into_iter()
        .map(|i| (ts[i] - ts_mean).powi(2))
        .sum::<f64>()
        / n as f64;
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn test_acf_acovf() {
        let data: Vec<f64> = vec![
            -213.0, -564.0, -35.0, -15.0, 141.0, 115.0, -420.0, -360.0, 203.0, -338.0, -431.0,
            194.0, -220.0, -513.0, 154.0, -125.0, -559.0, 92.0, -21.0, -579.0, -52.0, 99.0, -543.0,
            -175.0, 162.0, -457.0, -346.0, 204.0, -300.0, -474.0, 164.0, -107.0, -572.0, -8.0,
            83.0, -541.0, -224.0, 180.0, -420.0, -374.0, 201.0, -236.0, -531.0, 83.0, 27.0, -564.0,
            -112.0, 131.0, -507.0, -254.0, 199.0, -311.0, -495.0, 143.0, -46.0, -579.0, -90.0,
            136.0, -472.0, -338.0, 202.0, -287.0, -477.0, 169.0, -124.0, -568.0, 17.0, 48.0,
            -568.0, -135.0, 162.0, -430.0, -422.0, 172.0, -74.0, -577.0, -13.0, 92.0, -534.0,
            -243.0, 194.0, -355.0, -465.0, 156.0, -81.0, -578.0, -64.0, 139.0, -449.0, -384.0,
            193.0, -198.0, -538.0, 110.0, -44.0, -577.0, -6.0, 66.0, -552.0, -164.0, 161.0, -460.0,
            -344.0, 205.0, -281.0, -504.0, 134.0, -28.0, -576.0, -118.0, 156.0, -437.0, -381.0,
            200.0, -220.0, -540.0, 83.0, 11.0, -568.0, -160.0, 172.0, -414.0, -408.0, 188.0,
            -125.0, -572.0, -32.0, 139.0, -492.0, -321.0, 205.0, -262.0, -504.0, 142.0, -83.0,
            -574.0, 0.0, 48.0, -571.0, -106.0, 137.0, -501.0, -266.0, 190.0, -391.0, -406.0, 194.0,
            -186.0, -553.0, 83.0, -13.0, -577.0, -49.0, 103.0, -515.0, -280.0, 201.0, 300.0,
            -506.0, 131.0, -45.0, -578.0, -80.0, 138.0, -462.0, -361.0, 201.0, -211.0, -554.0,
            32.0, 74.0, -533.0, -235.0, 187.0, -372.0, -442.0, 182.0, -147.0, -566.0, 25.0, 68.0,
            -535.0, -244.0, 194.0, -351.0, -463.0, 174.0, -125.0, -570.0, 15.0, 72.0, -550.0,
            -190.0, 172.0, -424.0, -385.0, 198.0, -218.0, -536.0, 96.0,
        ];

        let autocorrelations: Vec<f64> = vec![
            1.00000000e+00,
            -3.07304801e-01,
            -7.40350266e-01,
            7.74689225e-01,
            2.05155438e-01,
            -8.98156108e-01,
            3.76063789e-01,
            6.32846512e-01,
            -7.69256070e-01,
            -1.24870822e-01,
            8.24513136e-01,
            -4.00433460e-01,
            -5.46316738e-01,
            7.31525640e-01,
            7.13826843e-02,
            -7.56326788e-01,
            4.02091425e-01,
            4.81864943e-01,
            -6.95651379e-01,
            -3.32642962e-02,
            7.00509687e-01,
            -4.09821551e-01,
            -4.29661465e-01,
            6.70177451e-01,
            -6.32534486e-04,
            -6.58335333e-01,
            4.15408913e-01,
            3.88528035e-01,
            -6.54277389e-01,
            2.61988532e-02,
            6.29088292e-01,
            -4.21577121e-01,
            -3.56941756e-01,
            6.40623681e-01,
            -5.24300195e-02,
            -6.01019436e-01,
            4.33174526e-01,
            3.21692640e-01,
            -6.36287268e-01,
            8.40102338e-02,
            5.76432704e-01,
            -4.49528938e-01,
            -2.82910789e-01,
            6.17116225e-01,
            -1.04847960e-01,
            -5.45169888e-01,
            4.53570519e-01,
            2.50418060e-01,
            -6.07894239e-01,
            1.39930070e-01,
        ];

        for i in 0..50 {
            assert_approx_eq!(acf(&data, i), autocorrelations[i]);
            assert_approx_eq!(acovf(&data, i) / acovf(&data, 0), acf(&data, i));
            assert!(acovf(&data, 0) >= acovf(&data, i).abs());
        }
    }
}