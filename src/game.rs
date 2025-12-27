use crate::components::*;
use crate::constants::VELOCITY;
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
        self.platform.draw(d);
        self.ball.draw(d);

        for p in &self.particles {
            p.draw(d);
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
            self.particles = Particle::spawn_particles(hit_point);
        }
    }

    fn handle_audio(&mut self) {
        if self.ball.is_dying()
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
