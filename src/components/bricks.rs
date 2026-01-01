use macroquad::prelude::*;

use crate::constants::BALL_RADIUS;

const WIDTH: f32 = 60.;
const HEIGHT: f32 = 20.;

pub struct Brick {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub active: bool,
    pub color: Color,
}

impl Brick {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Brick {
            pos,
            width: WIDTH,
            height: HEIGHT,
            active: true,
            color,
        }
    }

    pub fn die(&mut self) {
        self.active = false;
    }

    pub fn bound(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.width, self.height)
    }

    pub fn generate() -> Vec<Brick> {
        // TODO: Generate by Levels, maybe parse strings
        Brick::generate_simple(4, 10)
    }

    fn generate_simple(rows: u8, cols: u8) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let spacing = BALL_RADIUS;
        let start_y = 100.0;
        let start_x = (screen_width() - (cols as f32 * (WIDTH + spacing))) / 2.0;

        for r in 0..rows {
            for c in 0..cols {
                let pos = Vec2 {
                    x: start_x + c as f32 * (WIDTH + spacing),
                    y: start_y + r as f32 * (HEIGHT + spacing),
                };
                let mut color = WHITE;
                color.a = 0.5;
                bricks.push(Brick::new(pos, color));
            }
        }
        bricks
    }
}
