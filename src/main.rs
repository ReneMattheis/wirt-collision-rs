#![feature(get_many_mut)]
#![feature(map_many_mut)]
#![allow(dead_code)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod bsp;
mod collision_resolution;
mod constraint;
mod primitive;
mod util;
mod world;

use std::f64::consts::PI;

use graphics::color::{BLACK, RED, WHITE};
use primitive::{Body, Mass, Shape, Vec2};
use rand::Rng;
use world::{CollisionData, World};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    world: World,
    collisions: Vec<CollisionEffect>,
}

struct CollisionEffect {
    event: CollisionData,
    ttl: std::time::Duration,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y);

            for body in self.world.bodies() {
                let transform = transform.trans(body.position.x, -body.position.y);

                if let Shape::Circle { radius } = body.get_shape() {
                    let circ = ellipse::circle(0.0, 0.0, *radius);
                    ellipse(RED, circ, transform, gl);
                } else if let Shape::Square { edge_length } = body.get_shape() {
                    let square =
                        rectangle::square(-edge_length / 2.0, -edge_length / 2.0, *edge_length);
                    rectangle(RED, square, transform, gl);
                }
            }

            for collision in &self.collisions {
                let transform = c.transform.trans(x, y);

                let from = collision.event.collision.contact
                    + collision.event.collision.normal.scaled(20.0);
                let to = collision.event.collision.contact;

                line_from_to(WHITE, 1.0, [from.x, -from.y], [to.x, -to.y], transform, gl);

                let from_p = collision.event.collision.contact - Vec2::new_at(2.0, 2.0);
                let to_p = collision.event.collision.contact + Vec2::new_at(2.0, 2.0);

                rectangle_from_to(
                    WHITE,
                    [from_p.x, -from_p.y],
                    [to_p.x, -to_p.y],
                    transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut force_updates = vec![];
        for (entity, body) in self.world.entities() {
            let mass_value = if let Mass::Value(val) = *body.get_mass() {
                val
            } else {
                continue;
            };

            force_updates.push((*entity, Vec2::new_at(0.0, -300.0 * mass_value)));
        }

        for (entity, force) in force_updates {
            self.world.apply_force(entity, force);
        }

        let collisions = self
            .world
            .update(std::time::Duration::from_secs_f64(args.dt));

        let collisions_to_collect: Vec<CollisionData> = collisions.into_iter().collect();
        for collision in collisions_to_collect {
            self.collisions.push(CollisionEffect{event: collision, ttl: std::time::Duration::from_secs_f64(0.05)});
        }

        let delta_time = std::time::Duration::from_secs_f64(args.dt);
        for effect in self.collisions.iter_mut() {
            effect.ttl = effect.ttl.saturating_sub(delta_time);
        }

        self.collisions.retain(|e| !e.ttl.is_zero());
    }
}

fn main() {
    let mut world: World = World::new();

    let mut rng = rand::thread_rng();
    for _ in 0..40 {
        let x = rng.gen_range(-700.0..700.0);
        let y = rng.gen_range(-400.0..-200.0);
        let edge_length = rng.gen_range(20.0..45.0);

        let body = Body::new(
            Shape::Square { edge_length },
            Vec2::new_at(x, y),
            Mass::Infinity,
        );
        world.add(body);
    }

    for _ in 0..2000 {
        let x = rng.gen_range(-1000.0..1000.0);
        let y = rng.gen_range(500.0..4500.0);
        let dx = rng.gen_range(-50.0..50.0);
        let dy = rng.gen_range(-50.0..50.0);
        let radius = rng.gen_range(5.0..12.0);

        let mut body = Body::new(
            Shape::Circle { radius },
            Vec2::new_at(x, y),
            Mass::Value(PI * radius.powi(2)),
        );

        body.velocity.x = dx;
        body.velocity.y = dy;
        world.add(body);
    }

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("wirt-collision-rs", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        world,
        collisions: vec![],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
