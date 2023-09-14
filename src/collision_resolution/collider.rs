use crate::primitive::{Vec2, Mass, Body, Shape};

pub struct CircleCollider<'a> {
    pub position: &'a mut Vec2,
    pub radius: f64,
    pub mass: Mass,
}

pub struct SquareCollider<'a> {
    pub position: &'a mut Vec2,
    pub edge_length: f64,
    pub mass: Mass,
}

pub fn get_as_circle_collider(body: &mut Body) -> Option<CircleCollider> {
    if let Shape::Circle { radius } = *body.get_shape() {
        let mass = *body.get_mass();
        return Some(CircleCollider {
            position: &mut body.position,
            radius,
            mass,
        });
    }
    None
}

pub fn get_as_square_collider(body: &mut Body) -> Option<SquareCollider> {
    if let Shape::Square { edge_length } = *body.get_shape() {
        let mass = *body.get_mass();
        return Some(SquareCollider {
            position: &mut body.position,
            edge_length,
            mass,
        });
    }
    None
}