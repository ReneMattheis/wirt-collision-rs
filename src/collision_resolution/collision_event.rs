use crate::primitive::Vec2;

#[derive(Copy, Clone, Debug, Default)]
pub struct CollisionEvent {
    pub contact: Vec2,
    pub normal: Vec2,
    pub penetration_depth: f64,
}
