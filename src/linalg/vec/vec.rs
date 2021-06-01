use super::vops::*;
use crate::linalg::norm;
use crate::statistics::{argmax, argmin, max, mean, min, sample_std, sample_var, std, sum, var};
use impl_ops::*;
use std::convert::From;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Vector {
    v: Vec<f64>,
}

impl Vector {
    pub fn new() -> Vector {
        Self { v: Vec::new() }
    }
}

impl From<Vec<f64>> for Vector {
    fn from(v: Vec<f64>) -> Self {
        Self { v }
    }
}

impl IntoIterator for Vector {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl<'a> IntoIterator for &'a Vector {
    type Item = &'a f64;
    type IntoIter = std::slice::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.as_slice().into_iter()
    }
}

impl FromIterator<f64> for Vector {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = f64>,
    {
        Self {
            v: Vec::from_iter(iter),
        }
    }
}

impl Deref for Vector {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

// vector-vector ops
impl_op_ex!(+ |u: &Vector, v: &Vector| -> Vector {Vector::from( vadd(&u.v, &v.v) )});
impl_op_ex!(-|u: &Vector, v: &Vector| -> Vector { Vector::from(vsub(&u.v, &v.v)) });
impl_op_ex!(*|u: &Vector, v: &Vector| -> Vector { Vector::from(vmul(&u.v, &v.v)) });
impl_op_ex!(/ |u: &Vector, v: &Vector| -> Vector {Vector::from( vdiv(&u.v, &v.v) )});

// vector-float and float-vector ops
impl_op_ex_commutative!(+ |f: f64, v: &Vector| -> Vector { Vector::from(vsadd(&v.v, f)) });
impl_op_ex_commutative!(*|f: f64, v: &Vector| -> Vector { Vector::from(vsmul(&v.v, f)) });
impl_op_ex!(-|f: f64, v: &Vector| -> Vector { Vector::from(svsub(f, &v.v)) });
impl_op_ex!(-|v: &Vector, f: f64| -> Vector { Vector::from(vssub(&v.v, f)) });
impl_op_ex!(/ |f: f64, v: &Vector| -> Vector { Vector::from(svdiv(f, &v.v)) });
impl_op_ex!(/ |v: &Vector, f: f64| -> Vector { Vector::from(vsdiv(&v.v, f)) });

macro_rules! impl_unaryops_vector {
    ($fn: ident, $op: ident) => {
        impl Vector {
            pub fn $op(&self) -> Self {
                Self { v: $fn(&self.v) }
            }
        }
    };
}

impl_unaryops_vector!(vln, ln);
impl_unaryops_vector!(vln1p, ln_1p);
impl_unaryops_vector!(vlog10, log10);
impl_unaryops_vector!(vlog2, log2);
impl_unaryops_vector!(vexp, exp);
impl_unaryops_vector!(vexp2, exp2);
impl_unaryops_vector!(vsin, sin);
impl_unaryops_vector!(vcos, cos);
impl_unaryops_vector!(vtan, tan);
impl_unaryops_vector!(vsinh, sinh);
impl_unaryops_vector!(vcosh, cosh);
impl_unaryops_vector!(vtanh, tanh);
impl_unaryops_vector!(vasin, asin);
impl_unaryops_vector!(vacos, acos);
impl_unaryops_vector!(vatan, atan);
impl_unaryops_vector!(vasinh, asinh);
impl_unaryops_vector!(vacosh, acosh);
impl_unaryops_vector!(vatanh, atanh);
impl_unaryops_vector!(vsqrt, sqrt);
impl_unaryops_vector!(vcbrt, cbrt);
impl_unaryops_vector!(vabs, abs);
impl_unaryops_vector!(vfloor, floor);
impl_unaryops_vector!(vceil, ceil);
impl_unaryops_vector!(vtoradians, to_radians);
impl_unaryops_vector!(vtodegrees, to_degrees);
impl_unaryops_vector!(vrecip, recip);

macro_rules! impl_inner_fn {
    ($output_type: ident | $($fn: ident),+) => {
        $(
            impl Vector {
                pub fn $fn(&self) -> $output_type {
                    $fn(&self.v)
                }
            }
        )+
    };
}

impl_inner_fn!(
    f64 | norm,
    max,
    mean,
    min,
    std,
    sum,
    var,
    sample_std,
    sample_var
);

impl_inner_fn!(usize | argmin, argmax);