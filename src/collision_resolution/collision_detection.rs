
use crate::primitive::Vec2;

use super::collider::{CircleCollider, SquareCollider};
use super::collision_event::CollisionEvent;

fn dist_to_square_surface(angle: f64, edge_length: f64) -> f64 {
    let half_edge_len = edge_length / 2.0;
    let short_edge_len = half_edge_len / angle.cos();

    short_edge_len.hypot(half_edge_len)
}

fn nearest_point_to(square: &SquareCollider, point: &Vec2) -> Vec2 {
    let mut pos_in_square = *point;

    let half_edge_length = square.edge_length / 2.0;
    let edge_min_x = square.position.x - half_edge_length;
    let edge_max_x = square.position.x + half_edge_length;
    let edge_min_y = square.position.y - half_edge_length;
    let edge_max_y = square.position.y + half_edge_length;

    pos_in_square.x = pos_in_square.x.clamp(edge_min_x, edge_max_x);
    pos_in_square.y = pos_in_square.y.clamp(edge_min_y, edge_max_y);

    pos_in_square
}

pub fn detect_circle_vs_circle_collision(
    lhs: &CircleCollider,
    rhs: &CircleCollider,
) -> Option<CollisionEvent> {
    let pos_delta = *rhs.position - *lhs.position;
    let distance_squared = pos_delta.len_squared();
    let min_distance = lhs.radius + rhs.radius;
    let min_distance_squared = min_distance.powi(2);

    if distance_squared > min_distance_squared {
        return None;
    }

    let distance = pos_delta.len();
    let penetration_depth = min_distance - distance;

    let mut contact_point = pos_delta;
    contact_point.set_len(lhs.radius - (penetration_depth/2.0));
    contact_point += *lhs.position;

    let mut contact_normal = -pos_delta;
    contact_normal.normalize();

    Some(CollisionEvent {
        contact: contact_point,
        normal: contact_normal,
        penetration_depth,
    })
}

pub fn detect_square_vs_square_collision(
    lhs: &SquareCollider,
    rhs: &SquareCollider,
) -> Option<CollisionEvent> {
    let min_distance = (lhs.edge_length + rhs.edge_length) / 2.0;
    let position_delta = *rhs.position - *lhs.position;
    let overlap_x = min_distance - position_delta.x.abs();
    let overlap_y = min_distance - position_delta.y.abs();

    if overlap_x < 0.0 || overlap_y < 0.0 {
        return None;
    }

    let contact_pt_x = if lhs.position.x < rhs.position.x {
        lhs.position.x + lhs.edge_length / 2.0 - overlap_x / 2.0
    }
    else {
        lhs.position.x - lhs.edge_length / 2.0 + overlap_x / 2.0
    };
    
    let contact_pt_y = if lhs.position.y < rhs.position.y {
        lhs.position.y + lhs.edge_length / 2.0 - overlap_y / 2.0
    }
    else {
        lhs.position.y - lhs.edge_length / 2.0 + overlap_y / 2.0
    };
    
    let contact_point = Vec2::new_at(contact_pt_x, contact_pt_y);

    let penetration_depth;
    let mut contact_normal;

    if overlap_x > overlap_y {
        contact_normal = Vec2::new_at(0.0, -position_delta.y);
        penetration_depth = overlap_y;
    } else {
        contact_normal = Vec2::new_at(-position_delta.x, 0.0);
        penetration_depth = overlap_x;
    };

    contact_normal.normalize();

    Some(CollisionEvent {
        contact: contact_point,
        normal: contact_normal,
        penetration_depth,
    })
}

pub fn detect_circle_vs_square_collision(
    lhs: &CircleCollider,
    rhs: &SquareCollider,
) -> Option<CollisionEvent> {
    let nearest_point = nearest_point_to(rhs, lhs.position);
    let mut diff_to_nearest = nearest_point - *lhs.position;
    let mut dist_to_nearest_squared = diff_to_nearest.len_squared();

    if dist_to_nearest_squared == 0.0
    {
        // @todo: this is a really dirty workaround if the center of a circle is inside the square
        // as on those cases, nearest_point_to will just return the circle center
        dist_to_nearest_squared = 0.0001;
        diff_to_nearest = *rhs.position - *lhs.position;
    }

    let radius_squared = lhs.radius.powi(2);
    if radius_squared < dist_to_nearest_squared {
        return None;
    }

    let dist_to_nearest = dist_to_nearest_squared.sqrt();
    let mut contact_point = diff_to_nearest;
    let penetration_depth = lhs.radius - dist_to_nearest;
    contact_point.set_len(dist_to_nearest + (penetration_depth / 2.0));
    contact_point += *lhs.position;
    
    let mut contact_normal = -diff_to_nearest;
    contact_normal.normalize();

    Some(CollisionEvent {
        contact: contact_point,
        normal: contact_normal,
        penetration_depth,
    })
}
