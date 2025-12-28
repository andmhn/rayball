use crate::WINDOW_W;
use raylib::color::Color;
use raylib::math::{Rectangle, Vector2, rrect};

const WIDTH: f32 = 60.;
const HEIGHT: f32 = 20.;

pub struct Brick {
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
    pub active: bool,
    pub color: Color,
}

impl Brick {
    pub fn new(pos: Vector2, color: Color) -> Self {
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

    pub fn bound(&self) -> Rectangle {
        rrect(self.pos.x, self.pos.y, self.width, self.height)
    }

    pub fn generate() -> Vec<Brick> {
        // TODO: Generate by Levels, maybe parse strings
        Brick::generate_simple(4, 10)
    }

    fn generate_simple(rows: u8, cols: u8) -> Vec<Brick> {
        let mut bricks = Vec::new();
        let spacing = 20.0;
        let start_y = 100.0;
        let start_x = (WINDOW_W - (cols as f32 * (WIDTH + spacing))) / 2.0;

        for r in 0..rows {
            for c in 0..cols {
                let pos = Vector2 {
                    x: start_x + c as f32 * (WIDTH + spacing),
                    y: start_y + r as f32 * (HEIGHT + spacing),
                };
                bricks.push(Brick::new(pos, Color::RAYWHITE.alpha(0.5)));
            }
        }
        bricks
    }
}
