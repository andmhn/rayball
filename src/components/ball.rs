use crate::constants::{BALL_RADIUS, VELOCITY};
use raylib::math::Vector2;

#[derive(PartialEq)]
pub enum Status {
    Start,
    Running,
    Dead,
}

pub struct Ball {
    pub pos: Vector2,
    pub velocity: Vector2,
    pub status: Status,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vector2::zero(),
            velocity: Vector2::zero(),
            status: Status::Start,
            radius: BALL_RADIUS,
        }
    }

    pub fn die(&mut self) {
        if self.status != Status::Dead {
            self.status = Status::Dead;
            self.velocity = Vector2::zero();
        }
    }

    pub fn reset(&mut self) {
        self.status = Status::Start;
        self.velocity = Vector2::zero();
        self.pos = Vector2::zero();
    }

    pub fn launch(&mut self) {
        self.velocity.y = -VELOCITY;
        self.status = Status::Running;
    }
}
