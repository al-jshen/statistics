//! Various mathematical functions commonly used in statistics.

/// Calculates the standard [logistic function](https://en.wikipedia.org/wiki/Logistic_function)
///
/// ```
/// use approx_eq::assert_approx_eq;
/// use compute::functions::logistic;
/// use compute::distributions::{Exponential, Distribution};
/// let d = Exponential::new(5.).sample_iter(100 as usize);
/// d.iter().for_each(|x| {
///     assert_approx_eq!(logistic(*x) + logistic(-*x), 1.);
/// });
/// for i in 0..d.len() {
///     for j in i..d.len() {
///         if d[i] >= d[j] {
///             assert!(logistic(d[i]) >= logistic(d[j]));
///         }
///     }
/// }
/// assert_eq!(logistic(f64::NEG_INFINITY), 0.);
/// assert_eq!(logistic(0.), 0.5);
/// assert_eq!(logistic(f64::INFINITY), 1.);
/// ```
pub fn logistic(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}

/// Calculates the [logit function](https://en.wikipedia.org/wiki/Logit)
/// ```
/// use approx_eq::assert_approx_eq;
/// use compute::functions::{logistic, logit};
/// use compute::distributions::{Uniform, Distribution};
/// let d = Uniform::new(0., 1.).sample_iter(100 as usize);
/// d.iter().for_each(|x| {
///     assert_approx_eq!(*x, logistic(logit(*x)));
///     assert_approx_eq!(*x, logit(logistic(*x)));
/// });
/// for i in 0..d.len() {
///     for j in (i+1)..d.len() {
///         assert_approx_eq!(
///             logit(d[i]) - logit(d[j]),
///             ((d[i] / (1. - d[i])) / (d[j] / (1. - d[j]))).ln()
///         );
///     }
/// }
/// assert_eq!(logit(0.), f64::NEG_INFINITY);
/// assert_eq!(logit(0.5), 0.);
/// assert_eq!(logit(1.), f64::INFINITY);
/// ```
pub fn logit(p: f64) -> f64 {
    if p < 0. || p > 1. {
        panic!("p must be in [0, 1]");
    }
    (p / (1. - p)).ln()
}

/// Calculates the one-parameter Box-Cox transformation with some power parameter `lambda`.
pub fn boxcox(x: f64, lambda: f64) -> f64 {
    assert!(x > 0., "x must be positive");
    if lambda == 0. {
        x.ln()
    } else {
        (x.powf(lambda) - 1.) / lambda
    }
}

/// Calculates the two-parameter Box-Cox transformation with some power parameter `lambda` and some
/// shift parameter `alpha`.
pub fn boxcox_shifted(x: f64, lambda: f64, alpha: f64) -> f64 {
    assert!(x > alpha, "x must larger than alpha");
    if lambda == 0. {
        (x + alpha).ln()
    } else {
        ((x + alpha).powf(lambda) - 1.) / lambda
    }
}
