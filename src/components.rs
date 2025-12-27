use crate::constants::*;
use raylib::prelude::*;

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
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vector2 {
                x: WINDOW_W / 2.,
                y: WINDOW_H / 2.,
            },
            velocity: Vector2::zero(),
            status: Status::Start,
        }
    }

    pub fn pause(&mut self) {
        self.velocity = Vector2::zero();
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
        self.handle_wall_collisions();
    }

    fn handle_wall_collisions(&mut self) {
        let touched_down = (self.pos.y + BALL_RADIUS >= WINDOW_H) && (self.velocity.y > 0.0);
        if touched_down {
            self.pos.y = WINDOW_H - BALL_RADIUS;
            self.velocity.y *= -1.0;
            self.status = Status::Dead;
        }
        let touched_up = self.pos.y < BALL_RADIUS && self.velocity.y < 0.0;
        if touched_up {
            self.pos.y = BALL_RADIUS;
            self.velocity.y *= -1.0;
        }
        let touched_right = self.pos.x + BALL_RADIUS >= WINDOW_W && self.velocity.x > 0.0;
        if touched_right {
            self.pos.x = WINDOW_W - BALL_RADIUS;
            self.velocity.x *= -1.0;
        }
        let touched_left = self.pos.x < BALL_RADIUS && self.velocity.x < 0.0;
        if touched_left {
            self.pos.x = BALL_RADIUS;
            self.velocity.x *= -1.0;
        }
    }

    pub fn is_dead(&self) -> bool {
        (self.status == Status::Dead) && (self.velocity.y < 0.)
    }
}

pub struct Platform {
    pub pos: Vector2,
}

impl Platform {
    pub fn new() -> Self {
        Platform {
            pos: Vector2 {
                x: (WINDOW_W - PLATFORM_W) / 2.,
                y: WINDOW_H - PLATFORM_H,
            },
        }
    }

    pub fn move_left(&mut self, dt: f32) {
        self.pos.x -= 1000. * dt;
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }
    }

    pub fn move_right(&mut self, dt: f32) {
        self.pos.x += 1000. * dt;
        if self.pos.x + PLATFORM_W > WINDOW_W {
            self.pos.x = WINDOW_W - PLATFORM_W;
        }
    }
}
