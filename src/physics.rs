use crate::constants::*;
use raylib::prelude::*;

pub struct HitBox {
    pub rect: Rectangle,
}

#[allow(unused)]
impl HitBox {
    pub fn new(pos: Vector2, width: f32, height: f32) -> Self {
        let rect = rrect(pos.x, pos.y, width, height);
        HitBox { rect }
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.rect.check_collision_recs(other)
    }

    pub fn overlaps_circle(&self, circle_pos: Vector2, radius: f32) -> bool {
        self.rect.check_collision_circle_rec(circle_pos, radius)
    }

    pub fn is_off_screen(&self) -> bool {
        self.rect.y < 0. || self.rect.y > WINDOW_H || self.rect.x < 0. || self.rect.x > WINDOW_W
    }

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
