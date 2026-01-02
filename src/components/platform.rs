use crate::constants::*;
use macroquad::prelude::*;

pub struct Platform {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub fn new() -> Self {
        let width = screen_width() / 3.;
        let height = PLATFORM_H;
        let pos = Vec2 {
            x: (screen_width() - width) / 2.,
            y: screen_height() - height,
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
        if self.pos.x + self.width > screen_width() {
            self.pos.x = screen_width() - self.width;
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.width, self.height)
    }
}
