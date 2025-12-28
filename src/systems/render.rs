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
) {
    draw_ball_lives(d, dead_balls_pos, lives);
    draw_info_text(d, ball_status, lives);
}

pub fn draw_world(
    d: &mut RaylibDrawHandle,
    ball: &Ball,
    platform: &Platform,
    particles: &[Particle],
) {
    platform_draw(d, platform);
    ball_draw(ball, d);

    for p in particles {
        particle_draw(d, p);
    }
}

fn ball_draw(ball: &Ball, d: &mut RaylibDrawHandle) {
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

fn draw_info_text(d: &mut RaylibDrawHandle, ball_status: &Status, lives: u8) {
    match ball_status {
        Status::Start => {
            draw_text_center_x(d, "PRESS SPACE TO LAUNCH", INFO_POS_Y, 20, Color::GRAY)
        }
        Status::Dead => {
            if lives == 0 {
                draw_text_center_x(d, "GAME OVER", INFO_POS_Y - 100, 40, Color::RED);
                let text = "PRESS SPACE TO RESTART";
                draw_text_center_x(d, text, INFO_POS_Y, 20, Color::GRAY);
            }
        }
        _ => {}
    }
}

fn draw_ball_lives(d: &mut RaylibDrawHandle, dead_balls_pos: &Vec<Vector2>, lives: u8) {
    for pos in dead_balls_pos {
        d.draw_circle_v(pos, BALL_RADIUS, Color::RAYWHITE.alpha(0.2));
    }

    let spacing = BALL_RADIUS * 2.5;
    let margin = 30.;
    for i in 1..lives {
        let pos = Vector2 {
            x: margin + ((i as f32 - 1.) * spacing),
            y: margin,
        };
        d.draw_circle_v(pos, BALL_RADIUS, Color::RAYWHITE.alpha(0.2));
    }
}

fn draw_text_center_x(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let x = (WINDOW_W / 2.) as i32 - d.measure_text(text, font_size) / 2;
    d.draw_text(text, x, y, font_size, color);
}
