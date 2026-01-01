use crate::constants::{BALL_RADIUS, VELOCITY};
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum Status {
    Spawning,
    Start,
    Running,
    Dead,
}

pub struct Ball {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub status: Status,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            status: Status::Start,
            radius: BALL_RADIUS,
        }
    }

    pub fn die(&mut self) {
        if self.status != Status::Dead {
            self.status = Status::Dead;
            self.velocity = Vec2::ZERO;
        }
    }

    pub fn reset(&mut self) {
        self.status = Status::Start;
        self.velocity = Vec2::ZERO;
        self.pos = Vec2::ZERO;
    }

    pub fn launch(&mut self) {
        self.velocity.y = -VELOCITY;
        self.status = Status::Running;
    }
}
