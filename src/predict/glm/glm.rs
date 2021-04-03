use crate::prelude::{is_design, is_matrix, matmul, mean, solve, vadd, vdiv, vmul, vsub};

use super::ExponentialFamily;
use super::Formula;
use std::collections::HashMap;

/// Implements a [generalized linear model](https://en.wikipedia.org/wiki/Generalized_linear_model).
#[derive(Debug, Clone)]
pub struct GLM {
    pub family: ExponentialFamily,
    pub alpha: f64,
    pub tolerance: f64,
    pub weights: Option<Vec<f64>>,
    pub offsets: Option<Vec<f64>>,
    pub coef: Option<Vec<f64>>,
    pub deviance: Option<f64>,
    pub information_matrix: Option<Vec<f64>>,
}

impl GLM {
    /// Create a new general linear model with the given exponential family.
    /// `alpha` sets the ridge regression penalty strength, and `tolerance` sets the convergence
    /// tolerance.
    pub fn new(family: ExponentialFamily) -> Self {
        Self {
            family,
            alpha: 0.,
            tolerance: 1e-5,
            weights: None,
            offsets: None,
            coef: None,
            deviance: None,
            information_matrix: None,
        }
    }

    /// Set the lasso penalty strength.
    pub fn set_penalty(&mut self, alpha: f64) -> &mut Self {
        self.alpha = alpha;
        self
    }

    /// Set the convergence tolerance.
    pub fn set_tolerance(&mut self, tolerance: f64) -> &mut Self {
        self.tolerance = tolerance;
        self
    }

    /// Set the sample weights (usually measurement errors).
    pub fn set_weights(&mut self, weights: &[f64]) -> &mut Self {
        self.weights = Some(weights.to_vec());
        self
    }

    /// Set the offsets (usually used in Poisson regression models).
    pub fn set_offset(&mut self, offset: &[f64]) -> &mut Self {
        self.offsets = Some(offset.to_vec());
        self
    }

    fn fit_with_formula<'a>(&self, formula: Formula, data: HashMap<&'a str, Vec<f64>>) {
        todo!();
    }

    fn has_converged(&self, loss: f64, loss_previous: f64, tolerance: f64) -> bool {
        if loss_previous.is_infinite() {
            return false;
        }
        let rel_change = (loss - loss_previous).abs() / loss_previous;
        rel_change < tolerance
    }

    fn compute_dbeta(
        &self,
        x: &[f64],
        y: &[f64],
        mu: &[f64],
        dmu: &[f64],
        var: &[f64],
        weights: &[f64],
    ) -> Vec<f64> {
        let n = y.len();
        let p = is_matrix(x, n).unwrap();

        // weights * (y - mu) * (dmu / var)
        let working_residuals = vmul(&vmul(weights, &vsub(y, mu)), &vdiv(dmu, var));
        assert_eq!(working_residuals.len(), n);

        let mut dbeta = vec![0.; p];

        for i_n in 0..n {
            for i_p in 0..p {
                dbeta[i_p] -= x[i_n * p + i_p] * working_residuals[i_n];
            }
        }

        dbeta
    }

    fn compute_ddbeta(
        &self,
        x: &[f64],
        y: &[f64],
        dmu: &[f64],
        var: &[f64],
        weights: &[f64],
    ) -> Vec<f64> {
        let n = y.len();
        let p = is_matrix(x, n).unwrap();

        // weights * dmu**2 / var
        let working_weights = vdiv(&vmul(weights, &vmul(dmu, dmu)), var);
        let mut weighted_x = x.to_vec();

        for i_n in 0..n {
            for i_p in 0..p {
                weighted_x[i_n * p + i_p] *= working_weights[i_n];
            }
        }

        matmul(x, &weighted_x, n, n, true, false)
    }

    fn apply_dbeta_penalty(&self, dbeta: &mut [f64], coef: &[f64]) {
        for i in 1..coef.len() {
            dbeta[i] += coef[i];
        }
    }

    fn apply_ddbeta_penalty(&self, ddbeta: &mut [f64], n_predictors: usize) {
        for i in 0..n_predictors {
            ddbeta[i * n_predictors + i] += self.alpha;
        }
    }

    /// Fit the GLM using the [scoring algorithm](https://en.wikipedia.org/wiki/Score_(statistics)#Scoring_algorithm),
    /// which gives the maximumum likelihood estimate. It performs a maximum of `max_iter` iterations.
    /// Note that `x` must be a design matrix (i.e., the first column must contain all 1's).
    pub fn fit(&mut self, x: &[f64], y: &[f64], max_iter: usize) -> &mut Self {
        // check that the matrices are the right sizes
        let n = y.len();
        let p = is_matrix(x, n).unwrap();
        assert!(is_design(x, n), "x is not a design matrix");

        let weights = if let Some(w) = &self.weights {
            assert_eq!(w.len(), n, "wrong number of weights");
            w.to_vec()
        } else {
            vec![1.; n]
        };

        let initial_intercept = mean(y);
        let mut coef = vec![0.; p];
        coef[0] = initial_intercept;

        let mut penalized_deviance = f64::INFINITY;
        let mut is_converged = false;
        let mut n_iter = 0;

        let mut nu;
        let mut mu;
        let mut dmu;
        let mut var;
        let mut dbeta;
        let mut ddbeta;

        loop {
            // println!("{} {:?}", n_iter, coef);
            nu = matmul(x, &coef, n, p, false, false);
            if let Some(offset) = &self.offsets {
                assert_eq!(offset.len(), n, "wrong number of offsets");
                nu = vadd(&nu, offset);
            }
            // println!("{:?}", nu);

            mu = self.family.inv_link(&nu);
            // println!("mu {:?}", mu);
            dmu = self.family.d_inv_link(&nu, &mu);
            // println!("dmu {:?}", dmu);
            var = self.family.variance(&mu);
            // println!("var {:?}", var);

            dbeta = self.compute_dbeta(x, y, &mu, &dmu, &var, &weights);
            ddbeta = self.compute_ddbeta(x, y, &dmu, &var, &weights);

            // println!("dbeta {:?}", dbeta);
            // println!("ddbeta {:?}", ddbeta);

            // println!("coef before penalty {:?}", coef);
            if self.alpha > 0. {
                self.apply_dbeta_penalty(&mut dbeta, &coef);
                self.apply_ddbeta_penalty(&mut ddbeta, p);
            }

            // println!("dbeta {:?}", dbeta);
            // println!("ddbeta {:?}", ddbeta);

            // println!("solve {:?}", solve(&ddbeta, &dbeta));
            coef = vsub(&coef, &solve(&ddbeta, &dbeta));

            // println!("coef {:?}", coef);

            let penalized_deviance_previous = penalized_deviance;

            penalized_deviance = self.family.penalized_deviance(y, &mu, self.alpha, &coef);
            is_converged = self.has_converged(
                penalized_deviance,
                penalized_deviance_previous,
                self.tolerance,
            );
            n_iter += 1;

            if n_iter >= max_iter || is_converged {
                break;
            }
        }

        self.coef = Some(coef);
        self.deviance = Some(self.family.deviance(&y, &mu));
        self.information_matrix = Some(self.compute_ddbeta(x, y, &dmu, &var, &weights));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::design;
    use approx_eq::assert_approx_eq;

    /// This test is taken from [Wikipedia](https://en.wikipedia.org/wiki/Logistic_regression#Probability_of_passing_an_exam_versus_hours_of_study).
    #[test]
    fn test_glm_logistic() {
        let x = vec![
            0.50, 0.75, 1.00, 1.25, 1.50, 1.75, 1.75, 2.00, 2.25, 2.50, 2.75, 3.00, 3.25, 3.50,
            4.00, 4.25, 4.50, 4.75, 5.00, 5.50,
        ];
        let y = vec![
            0., 0., 0., 0., 0., 0., 1., 0., 1., 0., 1., 0., 1., 0., 1., 1., 1., 1., 1., 1.,
        ];
        let n = y.len();
        let xd = design(&x, n);

        let mut glm = GLM::new(ExponentialFamily::Bernoulli, 0., 1e-6);
        glm.fit(&xd, &y, 25);
        let coef = glm.coef.unwrap();
        assert_approx_eq!(coef[0], -4.0777, 1e-3);
        assert_approx_eq!(coef[1], 1.5046, 1e-3);
    }
}