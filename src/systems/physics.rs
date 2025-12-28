use crate::components::{Ball, Brick, Platform};
use crate::constants::{VELOCITY, WINDOW_H, WINDOW_W};
use crate::game::GameEvent;
use raylib::prelude::*;

pub fn resolve_ball_collisions(
    ball: &mut Ball,
    platform: &Platform,
    bricks: &mut [Brick],
) -> Vec<GameEvent> {
    let mut events = vec![];
    events.extend(handle_brick_collisions(bricks, ball));
    events.extend(handle_platform_collisions(ball, platform));
    events
}

pub fn update_ball_position(ball: &mut Ball, dt: f32) -> Vec<GameEvent> {
    ball.pos += ball.velocity * dt;
    let mut events = vec![];
    events.extend(handle_wall_collisions(ball));
    events
}

pub fn snap_ball_to_platform(ball: &mut Ball, platform: &Platform) {
    ball.pos.x = center_x(platform.bounds());
    ball.pos.y = platform.pos.y - ball.radius;
}

fn handle_platform_collisions(ball: &mut Ball, platform: &Platform) -> Option<GameEvent> {
    let p_bound = platform.bounds();
    let touching = p_bound.check_collision_circle_rec(ball.pos, ball.radius);
    if touching && ball.velocity.y > 0.0 {
        ball.velocity.y *= -1.0;
        ball.pos.y = platform.pos.y - ball.radius;

        // Calculate the bounce angle
        let diff = ball.pos.x - center_x(p_bound);
        ball.velocity.x = (diff / (p_bound.width / 2.0)) * VELOCITY;
        let hit_point = rvec2(ball.pos.x, platform.pos.y);

        return Some(GameEvent::BallHitPlatform(hit_point));
    }
    None
}

fn handle_brick_collisions(bricks: &mut [Brick], ball: &mut Ball) -> Option<GameEvent> {
    for brick in bricks.iter_mut().filter(|b| b.active) {
        let bound = brick.bound();
        if bound.check_collision_circle_rec(ball.pos, ball.radius) {
            let hitting_from_below = ball.velocity.y < 0.0 && ball.pos.y > bound.y + bound.height;
            let hitting_from_above = ball.velocity.y > 0.0 && ball.pos.y < bound.y;

            let hitting_from_left = ball.velocity.x > 0.0 && ball.pos.x < bound.x;
            let hitting_from_right = ball.velocity.x < 0.0 && ball.pos.x > bound.x + bound.width;

            if hitting_from_left || hitting_from_right {
                ball.velocity.x *= -1.0;
            } else if hitting_from_below || hitting_from_above {
                ball.velocity.y *= -1.0;
            } else {
                ball.velocity.y *= -1.0; // FALLBACK
            }

            brick.die();
            return Some(GameEvent::BrickCollision(ball.pos));
        }
    }
    None
}

fn handle_wall_collisions(ball: &mut Ball) -> Option<GameEvent> {
    let touched_down = (ball.pos.y + ball.radius >= WINDOW_H) && (ball.velocity.y > 0.0);
    if touched_down {
        ball.pos.y = WINDOW_H - ball.radius;
        ball.die();
        return Some(GameEvent::BallDropped);
    }
    let touched_up = ball.pos.y < ball.radius && ball.velocity.y < 0.0;
    if touched_up {
        ball.pos.y = ball.radius;
        ball.velocity.y *= -1.0;
    }
    let touched_right = ball.pos.x + ball.radius >= WINDOW_W && ball.velocity.x > 0.0;
    if touched_right {
        ball.pos.x = WINDOW_W - ball.radius;
        ball.velocity.x *= -1.0;
    }
    let touched_left = ball.pos.x < ball.radius && ball.velocity.x < 0.0;
    if touched_left {
        ball.pos.x = ball.radius;
        ball.velocity.x *= -1.0;
    }

    if touched_up || touched_left || touched_right {
        return Some(GameEvent::BallHitWall);
    }
    None
}

pub fn center_x(rect: Rectangle) -> f32 {
    rect.x + (rect.width / 2.0)
}

/// returns false if transition is done
pub fn transition_ball(ball: &mut Ball, platform: &Platform, dt: f32) -> bool {
    let destination = Vector2 {
        x: center_x(platform.bounds()),
        y: platform.pos.y - ball.radius,
    };
    if ball.pos.distance_to(destination) > 0.5 {
        ball.pos = ball.pos.lerp(destination, dt * 10.);
        return true;
    }
    false
}
