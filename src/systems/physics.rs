use crate::components::{Ball, Brick, Platform};
use crate::constants::VELOCITY;
use crate::game::GameEvent;
use macroquad::prelude::*;

pub fn resolve_ball_collisions(
    ball: &mut Ball,
    platform: &Platform,
    bricks: &mut [Brick],
) -> Vec<GameEvent> {
    let mut events = vec![];
    if let Some(e) = handle_brick_collisions(bricks, ball) {
        events.push(e);
    }
    if let Some(e) = handle_platform_collisions(ball, platform) {
        events.push(e);
    }
    events
}

pub fn update_ball_position(ball: &mut Ball, dt: f32) -> Vec<GameEvent> {
    ball.pos += ball.velocity * dt;
    let mut events = vec![];
    if let Some(e) = handle_wall_collisions(ball) {
        events.push(e);
    }
    events
}

pub fn snap_ball_to_platform(ball: &mut Ball, platform: &Platform) {
    ball.pos.x = center_x(platform.bounds());
    ball.pos.y = platform.pos.y - ball.radius;
}

fn handle_platform_collisions(ball: &mut Ball, platform: &Platform) -> Option<GameEvent> {
    let p_bound = platform.bounds();

    if circle_rect_collision(ball.pos, ball.radius, p_bound) && ball.velocity.y > 0.0 {
        ball.velocity.y *= -1.0;
        ball.pos.y = platform.pos.y - ball.radius;

        let diff = ball.pos.x - center_x(p_bound);
        ball.velocity.x = (diff / (p_bound.w / 2.0)) * VELOCITY;
        let hit_point = vec2(ball.pos.x, platform.pos.y);

        return Some(GameEvent::BallHitPlatform(hit_point));
    }
    None
}

fn handle_brick_collisions(bricks: &mut [Brick], ball: &mut Ball) -> Option<GameEvent> {
    for brick in bricks.iter_mut().filter(|b| b.active) {
        let bound = brick.bound();
        if circle_rect_collision(ball.pos, ball.radius, bound) {
            let _hitting_from_below = ball.velocity.y < 0.0 && ball.pos.y > bound.y + bound.h;
            let _hitting_from_above = ball.velocity.y > 0.0 && ball.pos.y < bound.y;

            let hitting_from_left = ball.velocity.x > 0.0 && ball.pos.x < bound.x;
            let hitting_from_right = ball.velocity.x < 0.0 && ball.pos.x > bound.x + bound.w;

            if hitting_from_left || hitting_from_right {
                ball.velocity.x *= -1.0;
            } else {
                ball.velocity.y *= -1.0;
            }

            brick.die();
            return Some(GameEvent::BrickCollision(ball.pos));
        }
    }
    None
}

fn handle_wall_collisions(ball: &mut Ball) -> Option<GameEvent> {
    let touched_down = (ball.pos.y + ball.radius >= screen_height()) && (ball.velocity.y > 0.0);
    if touched_down {
        ball.pos.y = screen_height() - ball.radius;
        ball.die();
        return Some(GameEvent::BallDropped);
    }

    let mut hit_wall = false;
    if ball.pos.y < ball.radius && ball.velocity.y < 0.0 {
        ball.pos.y = ball.radius;
        ball.velocity.y *= -1.0;
        hit_wall = true;
    }
    if ball.pos.x + ball.radius >= screen_width() && ball.velocity.x > 0.0 {
        ball.pos.x = screen_width() - ball.radius;
        ball.velocity.x *= -1.0;
        hit_wall = true;
    }
    if ball.pos.x < ball.radius && ball.velocity.x < 0.0 {
        ball.pos.x = ball.radius;
        ball.velocity.x *= -1.0;
        hit_wall = true;
    }

    if hit_wall {
        return Some(GameEvent::BallHitWall);
    }
    None
}

pub fn center_x(rect: Rect) -> f32 {
    rect.x + (rect.w / 2.0)
}

pub fn transition_ball(ball: &mut Ball, platform: &Platform, dt: f32) -> bool {
    let destination = vec2(center_x(platform.bounds()), platform.pos.y - ball.radius);
    if ball.pos.distance(destination) > 0.5 {
        ball.pos = ball.pos.lerp(destination, dt * 10.);
        return true;
    }
    false
}

// Helper function because Macroquad doesn't have a built-in Circle-to-Rect check
fn circle_rect_collision(center: Vec2, radius: f32, rect: Rect) -> bool {
    let closest_x = center.x.clamp(rect.x, rect.x + rect.w);
    let closest_y = center.y.clamp(rect.y, rect.y + rect.h);

    let distance_x = center.x - closest_x;
    let distance_y = center.y - closest_y;

    (distance_x * distance_x) + (distance_y * distance_y) < (radius * radius)
}
