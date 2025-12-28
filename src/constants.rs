use raylib::prelude::Color;

pub const WINDOW_W: f32 = 900.;
pub const WINDOW_H: f32 = 600.;
pub const INFO_POS_Y: i32 = WINDOW_H as i32 - 200;
pub const BG_COLOR: Color = Color::new(23, 25, 29, 255);

pub const VELOCITY: f32 = 900.0;
pub const BALL_RADIUS: f32 = 15.0;

pub const PLATFORM_W: f32 = 300.;
pub const PLATFORM_H: f32 = BALL_RADIUS;

pub const MAX_LIVES: u8 = 3;
