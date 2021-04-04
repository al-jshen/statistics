//! Implements [Cholesky decomposition](https://en.wikipedia.org/wiki/Cholesky_decomposition).

use crate::linalg::{dot, is_square, is_symmetric};

/// Computes the Cholesky decomposition of the matrix `a` using the Cholesky-Banachiewicz
/// algorithm.
pub fn cholesky(a: &[f64]) -> Vec<f64> {
    assert!(is_symmetric(a));
    let n = is_square(a).unwrap();

    let mut l = vec![0.; n * n];

    for i in 0..n {
        for j in 0..(i + 1) {
            let s = dot(&l[(j * n)..(j * n + j)], &l[(i * n)..(i * n + j)]);

            if i == j {
                l[i * n + j] = (a[i * n + i] - s).sqrt();
            } else {
                l[i * n + j] = (a[i * n + j] - s) / l[j * n + j];
            }
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn test_cholesky() {
        let a1 = vec![
            6., 3., 4., 8., 3., 6., 5., 1., 4., 5., 10., 7., 8., 1., 7., 25.,
        ];
        let l1 = cholesky(&a1);
        let b1 = vec![
            2.449489742783178,
            0.0,
            0.0,
            0.0,
            1.2247448713915892,
            2.1213203435596424,
            0.0,
            0.0,
            1.6329931618554523,
            1.414213562373095,
            2.309401076758503,
            0.0,
            3.2659863237109046,
            -1.4142135623730956,
            1.5877132402714704,
            3.1324910215354165,
        ];

        let a2 = vec![4., 12., -16., 12., 37., -43., -16., -43., 98.];
        let l2 = cholesky(&a2);
        let b2 = vec![2., 0., 0., 6., 1., 0., -8., 5., 3.];

        let a3 = vec![25., 15., -5., 15., 18., 0., -5., 0., 11.];
        let l3 = cholesky(&a3);
        let b3 = vec![5., 0., 0., 3., 3., 0., -1., 1., 3.];

        let a = [a1, a2, a3];
        let l = [l1, l2, l3];
        let b = [b1, b2, b3];

        for i in 0..3 {
            for j in 0..l.len() {
                assert_approx_eq!(l[i][j], b[i][j], 1e-2);
            }
        }
    }
}
