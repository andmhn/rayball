use crate::{DEFAULT_FONT, components::*, constants::BALL_RADIUS};
use macroquad::prelude::*;

pub fn draw_game_ui(lives: u8, ball_status: &Status, dead_balls_pos: &Vec<Vec2>, won: bool) {
    draw_ball_lives(dead_balls_pos, lives);
    draw_info_text(ball_status, lives, won);
}

pub fn draw_world(ball: &Ball, bricks: &[Brick], platform: &Platform, particles: &[Particle]) {
    platform_draw(platform);
    ball_draw(ball);
    for b in bricks.iter().filter(|b| b.active) {
        brick_draw(b);
    }
    for p in particles {
        particle_draw(p);
    }
}

fn brick_draw(brick: &Brick) {
    let b = brick;
    draw_rectangle(b.pos.x, b.pos.y, b.width, b.height, b.color);
    draw_rectangle_lines(
        b.pos.x,
        b.pos.y,
        b.width,
        b.height,
        2.,
        Color::new(1.0, 1.0, 1.0, 0.3),
    );
}

fn ball_draw(ball: &Ball) {
    if ball.status != Status::Dead {
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, YELLOW);
    }
}

fn platform_draw(platform: &Platform) {
    draw_rectangle(
        platform.pos.x,
        platform.pos.y,
        platform.width,
        platform.height,
        WHITE,
    );
}

fn particle_draw(particle: &Particle) {
    let mut color = particle.color;
    color.a = particle.life; // Apply alpha based on life
    draw_circle(particle.pos.x, particle.pos.y, 2., color);
}

fn draw_info_text(ball_status: &Status, lives: u8, won: bool) {
    let info_pos_y = screen_height() - 200.;
    let restart_text = "TOUCH / PRESS SPACE TO RESTART";
    if won {
        draw_text_center_x("GAME CLEARED", info_pos_y - 100., 40, LIME);
        draw_text_center_x(restart_text, info_pos_y, 20, GRAY);
        return;
    }
    match ball_status {
        Status::Start => draw_text_center_x("TOUCH / PRESS SPACE TO LAUNCH", info_pos_y as f32, 20, GRAY),
        Status::Dead => {
            if lives == 0 {
                draw_text_center_x("GAME OVER", info_pos_y - 100., 40, RED);
                draw_text_center_x(restart_text, info_pos_y, 20, GRAY);
            }
        }
        _ => {}
    }
}

const MARGIN: f32 = 30.;
const SPACING: f32 = BALL_RADIUS * 2.5;

pub fn get_ball_lives_pos(i: u8) -> Vec2 {
    vec2(MARGIN + ((i as f32 - 1.) * SPACING), MARGIN)
}

fn draw_ball_lives(dead_balls_pos: &Vec<Vec2>, lives: u8) {
    let ghost_color = Color::new(1.0, 1.0, 1.0, 0.2);
    for pos in dead_balls_pos {
        draw_circle(pos.x, pos.y, BALL_RADIUS, ghost_color);
    }

    for i in 1..lives {
        let pos = get_ball_lives_pos(i);
        draw_circle(pos.x, pos.y, BALL_RADIUS, ghost_color);
    }
}

fn draw_text_center_x(text: &str, y: f32, font_size: u16, color: Color) {
    let font = DEFAULT_FONT.get().expect("Font not loaded");
    let center = get_text_center(text, Some(font), font_size as u16, 1.0, 0.0);
    let x = (screen_width() / 2.) - center.x;
    draw_text_global(text, x, y, font_size, color);
}

pub fn draw_text_global(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let font = DEFAULT_FONT.get().expect("Font not loaded");
    let params = TextParams {
        font: Some(font),
        font_size,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x, y, params);
}
