use core::convert::From;

use crate::{primitive::{Body, Vec2, Shape}, util::lerp};
use super::partition::{Dimension, Partition};

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

#[derive(Debug)]
pub struct PartitionError {
    min: f64,
    max: f64,
    partition: f64,
}

impl Aabb {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        assert!(left <= right, "Left and right margins are inversed!");
        assert!(bottom <= top, "Top and bottom margins are inversed!");

        Self {
            top_left: Vec2::new_at(left, top),
            bottom_right: Vec2::new_at(right, bottom),
        }
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new_at(self.right(), self.bottom())
    }

    pub const fn bottom(&self) -> f64 {
        self.bottom_right.y
    }

    pub const fn top(&self) -> f64 {
        self.top_left.y
    }

    pub const fn right(&self) -> f64 {
        self.bottom_right.x
    }

    pub const fn left(&self) -> f64 {
        self.top_left.x
    }

    pub fn extent(&self) -> Vec2 {
        Vec2::new_at(self.right() - self.left(), self.top() - self.bottom())
    }

    pub fn intersects(&self, rhs: &Self) -> bool {
        self.left() <= rhs.right() &&
        rhs.left() <= self.right() &&
        self.bottom() <= rhs.top() &&
        rhs.bottom() <= self.top()
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new_at(
            lerp(self.left(), self.right(), 0.5),
            lerp(self.bottom(), self.top(), 0.5),
        )
    }

    pub fn merge(&mut self, other: &Self) -> Self {
        Self::new(
            self.left().min(other.left()),
            self.right().max(other.right()),
            self.top().max(other.top()),
            self.bottom().min(other.bottom()),
        )
    }

    pub fn merged(mut aabbs : impl Iterator<Item = Aabb>) -> Option<Aabb> {
        let mut merged_aabb: Aabb = aabbs.next()?;

        for aabb in aabbs.by_ref() {
            merged_aabb = merged_aabb.merge(&aabb);
        }

        Some(merged_aabb)
    }

    pub fn partition(&self, partition: Partition) -> Result<(Self, Self), PartitionError> {
        match partition.dimension {
            Dimension::X => {
                if partition.value < self.left() || partition.value > self.right() {
                    return Err(PartitionError {
                        min: self.left(),
                        max: self.right(),
                        partition: partition.value,
                    });
                }

                Ok((
                    Self::new(self.left(), partition.value, self.top(), self.bottom()),
                    Self::new(partition.value, self.right(), self.top(), self.bottom()),
                ))
            }
            Dimension::Y => {
                if partition.value < self.bottom() || partition.value > self.top() {
                    return Err(PartitionError {
                        min: self.bottom(),
                        max: self.top(),
                        partition: partition.value,
                    });
                }

                Ok((
                    Self::new(self.left(), self.right(), partition.value, self.bottom()),
                    Self::new(self.left(), self.right(), self.top(), partition.value),
                ))
            }
        }
    }
}

impl From<&Body> for Aabb {
    fn from(body: &Body) -> Self {
        match body.get_shape() {
            Shape::Circle { radius } => Self {
                top_left: body.position + Vec2::new_at(-*radius, *radius),
                bottom_right: body.position + Vec2::new_at(*radius,- *radius),
            },
            Shape::Square { edge_length } => Self {
                top_left: body.position + Vec2::new_at(-edge_length / 2.0, edge_length / 2.0),
                bottom_right: body.position + Vec2::new_at(edge_length / 2.0, -edge_length / 2.0),
            },
        }
    }
}
