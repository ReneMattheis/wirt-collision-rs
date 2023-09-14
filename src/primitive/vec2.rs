use std::ops::{Sub, SubAssign, AddAssign, Add, Mul, MulAssign, Neg};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub const fn new_at(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn new_polar(angle: f64, len: f64) -> Self {
        Self {
            x: angle.cos() * len,
            y: angle.sin() * len,
        }
    }

    pub fn len_squared(self) -> f64 {
        self.x.mul_add(self.x, self.y.powi(2))
    }

    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn divide_scalar(&mut self, scalar: f64) {
        assert!(scalar != 0.0);
        self.x /= scalar;
        self.y /= scalar;
    }

    pub fn multiply_scalar(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
    
    pub fn set_len(&mut self, len: f64) {
        assert!(self.x != 0.0 || self.y != 0.0);
        let current_len = self.len();
        self.multiply_scalar(len/current_len);
    }

    pub fn scaled(&self, scalar: f64) -> Self{
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn normalize(&mut self) {
        assert!(self.x != 0.0 || self.y != 0.0);
        self.divide_scalar(self.len());
    }

    pub fn scalar_product(&self, rhs: &Self) -> f64 {
        self.x.mul_add(rhs.x, self.y * rhs.y)
    }

    pub fn angle_rad(&self) -> f64 {
        self.y.atan2(self.x)
    }

}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output{
            x: -self.x,
            y: -self.y,
        }
    }
}