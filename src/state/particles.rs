use macroquad::prelude::Color;
use macroquad_toolkit::rng;

use crate::engine::{BuildingType, DroneState, GridPos};

use super::game_state::{Particle, PlanetState};

impl PlanetState {
    pub(super) fn spawn_particle(
        &mut self,
        position: (f32, f32),
        velocity: (f32, f32),
        life: f32,
        color: Color,
        size: f32,
    ) {
        self.particles.push(Particle {
            position,
            velocity,
            life,
            max_life: life,
            color,
            size,
        });
    }

    pub(super) fn spawn_resource_burst(&mut self) {
        let core_pos = match self.grid.find_core() {
            Some(pos) => pos,
            None => return,
        };
        let origin = (core_pos.x as f32, core_pos.y as f32);
        let count = 8;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let speed = rng::gen_range(0.6, 1.2);
            let velocity = (angle.cos() * speed, angle.sin() * speed);
            let life = rng::gen_range(0.35, 0.6);
            self.spawn_particle(
                origin,
                velocity,
                life,
                Color::new(1.0, 0.42, 0.21, 1.0),
                3.0,
            );
        }
    }

    pub(super) fn spawn_place_burst(&mut self, pos: GridPos) {
        let origin = (pos.x as f32, pos.y as f32);
        let count = 10;
        for index in 0..count {
            let angle = (index as f32 / count as f32) * std::f32::consts::TAU;
            let speed = rng::gen_range(0.6, 1.4);
            let velocity = (angle.cos() * speed, angle.sin() * speed);
            let life = rng::gen_range(0.25, 0.5);
            self.spawn_particle(origin, velocity, life, Color::new(0.0, 0.85, 1.0, 1.0), 2.5);
        }
    }

    pub(super) fn spawn_drone_trails(&mut self, delta_time: f32) {
        self.particle_timer += delta_time;
        if self.particle_timer < 0.08 {
            return;
        }
        self.particle_timer = 0.0;

        let drone_positions: Vec<(f32, f32)> = self
            .drones
            .drones()
            .iter()
            .filter(|drone| {
                drone.state == DroneState::MovingToCore || drone.state == DroneState::MovingToDrill
            })
            .map(|drone| drone.visual_position())
            .collect();

        for (x, y) in drone_positions {
            let jitter = (rng::gen_range(-0.2, 0.2), rng::gen_range(-0.2, 0.2));
            let velocity = (rng::gen_range(-0.4, 0.4), rng::gen_range(-0.4, 0.4));
            let life = rng::gen_range(0.25, 0.5);
            let color = Color::new(0.0, 0.85, 1.0, 1.0);
            self.spawn_particle((x + jitter.0, y + jitter.1), velocity, life, color, 2.0);
        }
    }

    pub(super) fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.position.0 += particle.velocity.0 * delta_time;
            particle.position.1 += particle.velocity.1 * delta_time;
            particle.life -= delta_time;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}
