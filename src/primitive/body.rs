use crate::Vec2;
use crate::Shape;
use crate::primitive::Mass;

#[derive(Copy, Clone, Debug)]
pub struct Body {
    shape: Shape,
    mass: Mass,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub force: Vec2,
    pub friction: f64,
}

impl Body {
    pub fn new(shape: Shape, position: Vec2, mass: Mass) -> Self {
        Self {
            shape,
            mass,
            position,
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            force: Vec2::default(),
            friction: 0.95,
        }
    }

    pub const fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub const fn get_mass(&self) -> &Mass {
        &self.mass
    }
}
