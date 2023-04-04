/// Fast inverse square root implementation.
/// Note that this returns a less approximate value than the default inv sqrt method, so it sacrifices accuracy for speed.
/// It should only be used in specific cases like the calculation of a vector magnitude.
#[inline]
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let y = f32::from_bits(0x5f3759df - (x.to_bits() >> 1));
    y * (1.5 - 0.5 * x * y * y)
}