pub fn lerp(from: f64, to: f64, mut t: f64) -> f64 {
    t = t.clamp(0.0, 1.0);
    from.mul_add(1.0 - t, to * t)
}

pub fn inverse_lerp(from: f64, to: f64, t: f64) -> f64 {
    let result = t - from / to - from;
    result.clamp(0.0, 1.0)
}