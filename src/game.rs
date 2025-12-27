use crate::components::*;
use crate::constants::VELOCITY;
use rand::Rng;
use raylib::prelude::*;

pub struct Game<'a> {
    ball: Ball,
    platform: Platform,
    audio_sample: Option<Sound<'a>>,
    particles: Vec<Particle>,
}

impl<'a> Game<'a> {
    pub fn new(audio_handle: Option<&'a RaylibAudio>) -> Self {
        let path = "assets/dropped.wav";
        let sound = audio_handle.and_then(|a| a.new_sound(path).ok());

        let mut game = Self {
            ball: Ball::new(),
            platform: Platform::new(),
            audio_sample: sound,
            particles: Vec::new(),
        };

        game.place_ball_on_platform();
        game
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        self.handle_input(rl, dt);
        self.physics_step(dt);
        self.handle_audio();
        self.cleanup_entities();
    }

    fn physics_step(&mut self, dt: f32) {
        self.handle_collision(dt);
        self.particles.iter_mut().for_each(|p| p.update(dt));
    }

    fn cleanup_entities(&mut self) {
        self.particles.retain(|p| p.life > 0.0);
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

        for p in &self.particles {
            d.draw_circle_v(p.pos, 2., p.color.alpha(1. * p.life));
        }
    }

    fn handle_collision(&mut self, dt: f32) {
        if self.ball.status == Status::Running {
            self.check_platform_collision_with_ball();
            self.ball.update(dt);
        }
    }

    fn place_ball_on_platform(&mut self) {
        self.ball.pos.x = self.platform.hitbox().center_x();
        self.ball.pos.y = self.platform.pos.y - self.ball.radius;
    }

    fn check_platform_collision_with_ball(&mut self) {
        let platform_hb = self.platform.hitbox();
        let overlaps = self.ball.collides_with_hitbox(&platform_hb);
        let moving_down = self.ball.velocity.y > 0.0;

        if overlaps && moving_down {
            self.ball.pos.y = platform_hb.rect.y - self.ball.radius;
            self.ball.velocity.y *= -1.0;

            // Calculate the bounce angle
            let diff = self.ball.pos.x - platform_hb.center_x();
            self.ball.velocity.x = (diff / (platform_hb.rect.width / 2.0)) * VELOCITY;

            let hit_point = rvec2(self.ball.pos.x, self.platform.pos.y);
            self.spawn_particles(hit_point);
        }
    }

    fn spawn_particles(&mut self, origin: Vector2) {
        let mut rng = rand::rng();

        for _ in 0..15 {
            let particle = Particle {
                color: Color::RAYWHITE,
                life: 1.0,
                pos: origin,
                vel: rvec2(
                    rng.random_range(-200.0..200.0),
                    rng.random_range(-400.0..-100.0),
                ),
            };
            self.particles.push(particle);
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
            self.ball.velocity.y = -VELOCITY;
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
