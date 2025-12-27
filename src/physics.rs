// use crate::constants::*;
use raylib::prelude::*;

pub struct HitBox {
    pub rect: Rectangle,
}

impl HitBox {
    pub fn new(pos: Vector2, width: f32, height: f32) -> Self {
        let rect = rrect(pos.x, pos.y, width, height);
        HitBox { rect }
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.rect.check_collision_recs(other)
    }

    // pub fn is_off_screen(&self) -> bool {
    //     self.pos.y > WINDOW_H || self.pos.x < 0. || self.pos.x > WINDOW_W
    // }

    pub fn centered_on(pos: Vector2, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x: pos.x - (width / 2.0),
            y: pos.y - (height / 2.0),
            width,
            height,
        }
    }

    pub fn center_x(&self) -> f32 {
        self.rect.x + (self.rect.width / 2.0)
    }
}
