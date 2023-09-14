use crate::primitive::Body;

use super::{
    collider::{get_as_circle_collider, get_as_square_collider},
    collision_event::CollisionEvent,
    collision_detection::{
        detect_circle_vs_circle_collision, detect_circle_vs_square_collision,
        detect_square_vs_square_collision,
    },
};

const OVERLAP_DISPLACEMENT_DISTANCE: f64 = 0.01;

pub fn detect_collision(lhs: &mut Body, rhs: &mut Body) -> Option<CollisionEvent> {
    let mut collision = None;

    if let (Some(lhs), Some(rhs)) = (get_as_circle_collider(lhs), get_as_circle_collider(rhs)) {
        collision = detect_circle_vs_circle_collision(&lhs, &rhs);
    };

    if let (Some(lhs), Some(rhs)) = (get_as_square_collider(lhs), get_as_square_collider(rhs)) {
        collision = detect_square_vs_square_collision(&lhs, &rhs);
    };

    if let (Some(lhs), Some(rhs)) = (get_as_circle_collider(lhs), get_as_square_collider(rhs)) {
        collision = detect_circle_vs_square_collision(&lhs, &rhs);
    };

    if let (Some(lhs), Some(rhs)) = (get_as_square_collider(lhs), get_as_circle_collider(rhs)) {
        let mut temp_collision = detect_circle_vs_square_collision(&rhs, &lhs);
        if let Some(collision) = &mut temp_collision {
            collision.normal = -collision.normal;
        }
        collision = temp_collision;
    };

    collision
}

pub fn resolve_collision(lhs: &mut Body, rhs: &mut Body, collision: &CollisionEvent) {
    if rhs.position == lhs.position {
        // Correction for when both entities share the exact same Vec2
        lhs.position.x -= OVERLAP_DISPLACEMENT_DISTANCE;
        rhs.position.x += OVERLAP_DISPLACEMENT_DISTANCE;
    }

    let lhs_inverse_mass = lhs.get_mass().get_inverse();
    let rhs_inverse_mass = rhs.get_mass().get_inverse();
    let total_inverse_mass = lhs_inverse_mass + rhs_inverse_mass;

    if total_inverse_mass <= 0.0 {
        // infinite masses can not be moved, so return early
        return;
    }

    let lhs_move_per_inverse_mass =
        collision.normal * (collision.penetration_depth / total_inverse_mass);
    let rhs_move_per_inverse_mass =
        collision.normal * (collision.penetration_depth / -total_inverse_mass);

    lhs.position += lhs_move_per_inverse_mass.scaled(lhs_inverse_mass);
    //rhs.position += rhs_move_per_inverse_mass.scaled(rhs_inverse_mass);

    // Do post-processing for forced separation where the velocity based
    // position correction was unsuccessful, e.g. due to bodies which
    // were already spawned with no velocity but intersecting
    //if let Some(collision) = detect_collision(lhs, rhs) {
    //    if lhs.get_mass().is_infinite() {
    //    } else if rhs.get_mass().is_infinite() {
    //        lhs.position += collision.normal.scaled(collision.penetration_depth);
    //    } else {
    //        lhs.position += collision.normal.scaled(collision.penetration_depth / 2.0);
    //    }
    //}
}

pub fn detect_and_resolve_collision(lhs: &mut Body, rhs: &mut Body) -> Option<CollisionEvent> {
    if rhs.position == lhs.position {
        // Correction for when both entities share the exact same Vec2
        lhs.position.x -= OVERLAP_DISPLACEMENT_DISTANCE;
        rhs.position.x += OVERLAP_DISPLACEMENT_DISTANCE;
    }

    let collision = detect_collision(lhs, rhs)?;
    resolve_collision(lhs, rhs, &collision);
    Some(collision)
}
