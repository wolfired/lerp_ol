use std::ops::{Add, Mul, Sub};

pub trait FloatOps:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>
{
}

pub trait Float: FloatOps {
    fn one() -> Self;
}

impl<T: Float> FloatOps for T {}

impl Float for f32 {
    fn one() -> Self {
        1.0
    }
}

impl Float for f64 {
    fn one() -> Self {
        1.0
    }
}

pub fn lerp<F: Float>(v0: F, v1: F, t: F) -> F {
    (F::one() - t) * v0 + t * v1
}
