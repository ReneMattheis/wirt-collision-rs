use std::ops::Neg;

use super::aabb::Aabb;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dimension {
    X,
    Y,
}

impl Neg for Dimension {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self == Self::X {
            Self::Y
        } else {
            Self::X
        }
    }
}

#[derive(Clone, Copy)]
pub struct Partition {
    pub dimension: Dimension,
    pub value: f64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Bucket {
    Below,
    Intersecting,
    Above,
}

impl Partition {
    pub fn classify(&self, aabb: &Aabb) -> Bucket {
        match self.dimension {
            Dimension::X => {
                if aabb.right() < self.value {
                    Bucket::Below
                } else if aabb.left() > self.value {
                    Bucket::Above
                } else {
                    Bucket::Intersecting
                }
            }
            Dimension::Y => {
                if aabb.top() < self.value {
                    Bucket::Below
                } else if aabb.bottom() > self.value {
                    Bucket::Above
                } else {
                    Bucket::Intersecting
                }
            }
        }
    }
}
