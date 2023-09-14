use std::hash::Hash;
use std::{collections::HashMap, time::Instant};

use crate::bsp::aabb::Aabb;
use crate::bsp::entry::Entry;
use crate::bsp::tree::Tree;
use crate::collision_resolution::{self, CollisionEvent};
use crate::primitive::Body;
use crate::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub struct Entity {
    id: usize,
}

pub struct World {
    bodies: HashMap<Entity, Body>,
    last_id: usize,
}

pub struct CollisionData {
    pub ids: [Entity; 2],
    pub collision: CollisionEvent,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn add(&mut self, body: Body) -> Entity {
        self.last_id += 1;
        self.bodies.insert(Entity { id: self.last_id }, body);
        Entity { id: self.last_id }
    }

    pub fn remove(&mut self, entity: Entity) {
        self.bodies.remove(&entity);
    }

    pub fn apply_force(&mut self, entity: Entity, force: Vec2) {
        let body = self.bodies.get_mut(&entity);
        if let Some(body) = body {
            body.force += force;
        }
    }

    pub fn body(&self, entity: Entity) -> Option<&Body> {
        self.bodies.get(&entity)
    }

    pub fn bodies(&self) -> impl Iterator<Item = &Body> {
        self.bodies.values()
    }

    pub fn ids(&self) -> impl Iterator<Item = Entity> + '_ {
        self.bodies.keys().copied()
    }

    pub fn update(&mut self, dt: std::time::Duration) -> Vec<CollisionData> {
        let delta = dt.as_secs_f64();

        for body in self.bodies.values_mut() {
            body.position += body.velocity * delta;

            let inverse_mass = body.get_mass().get_inverse();
            body.acceleration = body.force * inverse_mass;

            body.velocity += body.acceleration * delta;
            body.velocity *= body.friction.powf(delta);
        }

        let collision_data = self.resolve_collisions();
        for collision in &collision_data {
            let ids: [&Entity; 2] = [&collision.ids[0], &collision.ids[1]];
            let [lhs, rhs] = self.bodies.get_many_mut(ids).unwrap();
            Self::resolve_contact_velocity(lhs, rhs, &collision.collision.normal);
        }

        for body in self.bodies.values_mut() {
            body.force = Vec2::default();
        }

        collision_data
    }

    fn calculate_separating_velocity(
        lhs_velocity: &Vec2,
        rhs_velocity: &Vec2,
        contact_normal: &Vec2,
    ) -> f64 {
        let relative_velocity = *lhs_velocity - *rhs_velocity;
        relative_velocity.scalar_product(contact_normal)
    }

    fn resolve_contact_velocity(lhs: &mut Body, rhs: &mut Body, contact_normal: &Vec2) {
        let separating_velocity =
            Self::calculate_separating_velocity(&lhs.velocity, &rhs.velocity, contact_normal);
        if separating_velocity > 0.0 {
            return;
        }

        let restitution = 0.9;
        let new_separating_velocity = -separating_velocity * restitution;
        let delta_velocity = new_separating_velocity - separating_velocity;

        let lhs_inverse_mass = lhs.get_mass().get_inverse();
        let rhs_inverse_mass = rhs.get_mass().get_inverse();
        let total_inverse_mass = lhs_inverse_mass + rhs_inverse_mass;
        if total_inverse_mass <= 0.0 {
            // If we only have infinite masses, then nothing to do
            return;
        }

        let impulse = delta_velocity / total_inverse_mass;
        let impulse_per_mass = contact_normal.scaled(impulse);

        lhs.velocity += impulse_per_mass.scaled(lhs_inverse_mass);
        rhs.velocity += impulse_per_mass.scaled(-rhs_inverse_mass);
    }

    fn resolve_collisions(&mut self) -> Vec<CollisionData> {
        let mut collisions: Vec<CollisionData> = vec![];
        let timer = Instant::now();

        let entries: Vec<_> = self
            .bodies
            .iter()
            .map(|(k, v)| Entry {
                id: k.id,
                aabb: Aabb::from(v),
            })
            .collect();

        let bsp_tree = Tree::new_with(entries);

        let potential_collisions = bsp_tree.internal_collisions();
        for (lhs, rhs) in potential_collisions {
            let ids = [Entity { id: lhs }, Entity { id: rhs }];
            let id_refs = [&ids[0], &ids[1]];
            let [lhs, rhs] = self.bodies.get_many_mut(id_refs).unwrap();
            let collision = collision_resolution::detect_collision(lhs, rhs);
            if let Some(collision) = collision {
                collisions.push(CollisionData { ids, collision });
            }
        }

        for collision in &collisions {
            let ids = [&collision.ids[0], &collision.ids[1]];
            let [lhs, rhs] = self.bodies.get_many_mut(ids).unwrap();
            collision_resolution::resolve_collision(lhs, rhs, &collision.collision);
        }

        let duration = timer.elapsed();
        println!("{}us", duration.as_micros());

        collisions
    }
}
