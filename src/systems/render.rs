use crate::{
    components::*,
    constants::{BALL_RADIUS, INFO_POS_Y, WINDOW_W},
};
use raylib::prelude::*;

pub fn draw_game_ui(
    d: &mut RaylibDrawHandle,
    lives: u8,
    ball_status: &Status,
    dead_balls_pos: &Vec<Vector2>,
    won: bool,
) {
    draw_ball_lives(d, dead_balls_pos, lives);
    draw_info_text(d, ball_status, lives, won);
}

pub fn draw_world(
    d: &mut RaylibDrawHandle,
    ball: &Ball,
    bricks: &[Brick],
    platform: &Platform,
    particles: &[Particle],
) {
    platform_draw(d, platform);
    ball_draw(d, ball);
    for b in bricks.iter().filter(|b| b.active) {
        brick_draw(d, b);
    }
    for p in particles {
        particle_draw(d, p);
    }
}

fn brick_draw(d: &mut RaylibDrawHandle, brick: &Brick) {
    let b = brick;
    d.draw_rectangle_v(b.pos, rvec2(b.width, b.height), b.color);
    d.draw_rectangle_lines_ex(b.bound(), 2., Color::WHITE.alpha(0.3));
}

fn ball_draw(d: &mut RaylibDrawHandle, ball: &Ball) {
    if ball.status != Status::Dead {
        d.draw_circle_v(ball.pos, ball.radius, Color::YELLOW);
    }
}

fn platform_draw(d: &mut RaylibDrawHandle, platform: &Platform) {
    d.draw_rectangle_v(
        platform.pos,
        rvec2(platform.width, platform.height),
        Color::RAYWHITE,
    );
}

fn particle_draw(d: &mut RaylibDrawHandle, particle: &Particle) {
    d.draw_circle_v(particle.pos, 2., particle.color.alpha(particle.life));
}

fn draw_info_text(d: &mut RaylibDrawHandle, ball_status: &Status, lives: u8, won: bool) {
    let restart_text = "PRESS SPACE TO RESTART";
    if won {
        draw_text_center_x(d, "GAME CLEARED", INFO_POS_Y - 100, 40, Color::LIME);
        draw_text_center_x(d, restart_text, INFO_POS_Y, 20, Color::GRAY);
        return;
    }
    match ball_status {
        Status::Start => {
            draw_text_center_x(d, "PRESS SPACE TO LAUNCH", INFO_POS_Y, 20, Color::GRAY)
        }
        Status::Dead => {
            if lives == 0 {
                draw_text_center_x(d, "GAME OVER", INFO_POS_Y - 100, 40, Color::RED);
                draw_text_center_x(d, restart_text, INFO_POS_Y, 20, Color::GRAY);
            }
        }
        _ => {}
    }
}

const MARGIN: f32 = 30.;
const SPACING: f32 = BALL_RADIUS * 2.5;

pub fn get_ball_lives_pos(i: u8) -> Vector2 {
    Vector2 {
        x: MARGIN + ((i as f32 - 1.) * SPACING),
        y: MARGIN,
    }
}

fn draw_ball_lives(d: &mut RaylibDrawHandle, dead_balls_pos: &Vec<Vector2>, lives: u8) {
    for pos in dead_balls_pos {
        d.draw_circle_v(pos, BALL_RADIUS, Color::RAYWHITE.alpha(0.2));
    }

    for i in 1..lives {
        let pos = get_ball_lives_pos(i);
        d.draw_circle_v(pos, BALL_RADIUS, Color::RAYWHITE.alpha(0.2));
    }
}

fn draw_text_center_x(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let x = (WINDOW_W / 2.) as i32 - d.measure_text(text, font_size) / 2;
    d.draw_text(text, x, y, font_size, color);
}
