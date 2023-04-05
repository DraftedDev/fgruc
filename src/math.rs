/// Fast inverse square root implementation.
/// Note that this returns a less approximate value than the default inv sqrt method, so it sacrifices accuracy for speed.
/// It should only be used in specific cases like the calculation of a vector magnitude.
#[inline]
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let y = f32::from_bits(0x5f3759df - (x.to_bits() >> 1));
    y * (1.5 - 0.5 * x * y * y)
}

/// A faster implementation of sin() function.
/// Sacrifices accuracy for speed.
pub fn fast_sin(x: f32) -> f32 {
    const A: f32 = 1.27323954;
    const B: f32 = 0.405284735;
    const C: f32 = 0.225;
    let y = A * x - B * x.abs() * x;
    C * (y.abs() - y) + y
}

/// A faster implementation of cos() function.
/// Sacrifices accuracy for speed.
pub fn fast_cos(x: f32) -> f32 {
    const A: f32 = 1.27323954;
    const B: f32 = 0.405284735;
    const C: f32 = 0.225;
    let y = A * x - B * x.abs() * x;
    C * (y.abs() - y) - y * x.signum() + x
}

/// A faster implementation of tan() function.
/// Sacrifices accuracy for speed.
#[inline]
pub fn fast_tan(x: f32) -> f32 {
    fast_sin(x) / fast_cos(x)
}