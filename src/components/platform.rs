use crate::constants::*;
use raylib::math::{Rectangle, Vector2, rrect};

pub struct Platform {
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub fn new() -> Self {
        let width = PLATFORM_W;
        let height = PLATFORM_H;
        let pos = Vector2 {
            x: (WINDOW_W - width) / 2.,
            y: WINDOW_H - height,
        };
        Platform { pos, width, height }
    }

    pub fn move_left(&mut self, dt: f32) {
        self.pos.x -= 1000. * dt;
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }
    }

    pub fn move_right(&mut self, dt: f32) {
        self.pos.x += 1000. * dt;
        if self.pos.x + self.width > WINDOW_W {
            self.pos.x = WINDOW_W - self.width;
        }
    }

    pub fn bounds(&self) -> Rectangle {
        rrect(self.pos.x, self.pos.y, self.width, self.height)
    }
}
