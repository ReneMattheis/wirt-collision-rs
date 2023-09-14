
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mass {
    Value(f64),
    Infinity,
}

impl Mass {
    pub fn is_infinite(&self) -> bool {
        *self == Self::Infinity
    }

    pub fn get_inverse(&self) -> f64 {
        match self {
            Self::Value(mass) => 1.0/mass,
            Self::Infinity => 0.0,
        }
    }
}
