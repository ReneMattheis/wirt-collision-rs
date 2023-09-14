use crate::{
    primitive::{Body, Mass},
    world::Entity,
};

pub trait Constraint {
    fn get_first_entity(&self) -> Entity;
    fn get_second_entity(&self) -> Entity;
    fn process(&self, lhs: &mut Body, rhs: &Body, delta: f64);
}

pub struct FakeSpring {
    pub entity1: Entity,
    pub entity2: Entity,
    pub damping: f64,
    pub spring_constant: f64,
}

impl FakeSpring {
    pub fn new(entity1: Entity, entity2: Entity, damping: f64) -> Self {
        Self {
            entity1,
            entity2,
            damping,
            spring_constant: 100.0,
        }
    }

    pub fn process(&self, lhs: &mut Body, rhs: &Body, delta: f64) {
        if lhs.get_mass().is_infinite() {
            return;
        }

        let mass_value = if let Mass::Value(val) = *lhs.get_mass() {
            val
        } else {
            return;
        };

        let position = lhs.position - rhs.position;

        let gamma = 0.5 * (4.0 * self.spring_constant - self.damping.powi(2)).sqrt();
        if gamma == 0.0 {
            return;
        }

        let c = position * (self.damping / (2.0 * gamma)) + lhs.velocity * (1.0 / gamma);

        let mut target = position * (gamma * delta).cos() + c * (gamma * delta).sin();
        target *= (-0.5 * delta * self.damping).exp();

        let accel = (target - position) * (1.0 / (delta.powi(2))) - lhs.velocity * (1.0 / delta);
        lhs.force += accel * mass_value;
    }
}

impl Constraint for FakeSpring {
    fn get_first_entity(&self) -> Entity {
        self.entity1
    }
    
    fn get_second_entity(&self) -> Entity {
        self.entity2
    }

    fn process(&self, lhs: &mut Body, rhs: &Body, delta: f64) {
        self.process(lhs, rhs, delta);
    }
}

pub struct BungeeSpring {
    pub entity1: Entity,
    pub entity2: Entity,
    pub distance: f64,
    pub spring_constant: f64,
}

impl BungeeSpring {
    pub fn new(entity1: Entity, entity2: Entity, distance: f64) -> Self {
        Self {
            entity1,
            entity2,
            distance,
            spring_constant: 100.0,
        }
    }

    pub fn process(&self, lhs: &mut Body, rhs: &Body, _delta: f64) {
        let mut force = lhs.position - rhs.position;
        let magnitude = force.len();

        if magnitude <= self.distance {
            return;
        }

        let magnitude = self.spring_constant * (self.distance - magnitude);
        force.normalize();
        force *= -magnitude;
        lhs.force -= force;
    }
}

impl Constraint for BungeeSpring {
    fn get_first_entity(&self) -> Entity {
        self.entity1
    }

    fn get_second_entity(&self) -> Entity {
        self.entity2
    }

    fn process(&self, lhs: &mut Body, rhs: &Body, delta: f64) {
        self.process(lhs, rhs, delta);
    }
}
