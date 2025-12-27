use crate::components::*;
use crate::constants::{VELOCITY, WINDOW_H};
use raylib::prelude::*;

pub struct Game<'a> {
    ball: Ball,
    platform: Platform,
    audio_sample: Option<Sound<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(audio_handle: Option<&'a RaylibAudio>) -> Self {
        let path = "assets/dropped.wav";
        let sound = audio_handle.and_then(|a| a.new_sound(path).ok());

        let mut game = Self {
            ball: Ball::new(),
            platform: Platform::new(),
            audio_sample: sound,
        };

        game.place_ball_on_platform();
        game
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        self.handle_collision(dt);
        self.handle_input(rl, dt);
        self.handle_audio();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.platform.pos,
            rvec2(self.platform.width, self.platform.height),
            Color::RAYWHITE,
        );
        if self.ball.status == Status::Dead {
            d.draw_circle_v(self.ball.pos, self.ball.radius, Color::YELLOW.alpha(0.5));
        } else {
            d.draw_circle_v(self.ball.pos, self.ball.radius, Color::YELLOW);
        }
    }

    fn handle_collision(&mut self, dt: f32) {
        if self.ball.status == Status::Running {
            self.ball.update(dt);
            self.check_platform_collision_with_ball();
        }
    }

    fn place_ball_on_platform(&mut self) {
        self.ball.pos.x = self.platform.pos.x + self.platform.width / 2.;
        self.ball.pos.y = WINDOW_H - self.platform.height - self.ball.radius;
    }

    fn check_platform_collision_with_ball(&mut self) {
        let platform_hb = self.platform.hitbox();
        let ball_bounds = self.ball.bounds();

        if platform_hb.overlaps(&ball_bounds) && self.ball.velocity.y > 0.0 {
            self.ball.pos.y = platform_hb.rect.y - self.ball.radius;
            self.ball.velocity.y *= -1.0;

            // Calculate the bounce angle
            let diff = self.ball.pos.x - platform_hb.center_x();
            self.ball.velocity.x = (diff / (platform_hb.rect.width / 2.0)) * VELOCITY;
        }
    }

    fn handle_audio(&mut self) {
        if self.ball.is_dead()
            && let Some(s) = &self.audio_sample
        {
            s.play();
            self.ball.pause();
        }
    }

    fn handle_input(&mut self, rl: &RaylibHandle, dt: f32) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.ball.status == Status::Start {
            self.ball.velocity.y = VELOCITY;
            self.ball.status = Status::Running;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.ball.status == Status::Dead {
            self.place_ball_on_platform();
            self.ball.status = Status::Start;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.platform.move_left(dt);
            if self.ball.status == Status::Start {
                self.place_ball_on_platform();
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.platform.move_right(dt);
            if self.ball.status == Status::Start {
                self.place_ball_on_platform();
            }
        }
    }
}
