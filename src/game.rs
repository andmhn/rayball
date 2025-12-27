use crate::components::*;
use crate::constants::{MAX_LIVES, VELOCITY};
use raylib::prelude::*;

pub enum GameEvent {
    BallHitWall,
    BallDropped,
}

pub struct Game<'a> {
    ball: Ball,
    platform: Platform,
    particles: Vec<Particle>,
    sounds: SoundManager<'a>,
    lives: u8,
    dead_balls_pos: Vec<Vector2>,
}

impl<'a> Game<'a> {
    pub fn new(sounds: SoundManager<'a>) -> Self {
        let mut game = Self {
            ball: Ball::new(),
            platform: Platform::new(),
            particles: Vec::new(),
            sounds,
            lives: MAX_LIVES,
            dead_balls_pos: Vec::new(),
        };

        game.place_ball_on_platform();
        game
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        self.handle_input(rl, dt);
        self.physics_step(dt);
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
        self.ball.draw(d, Color::YELLOW);

        for p in &self.particles {
            p.draw(d);
        }
        match self.ball.status {
            Status::Start => d.draw_text("PRESS SPACE TO LAUNCH", 300, 400, 20, Color::GRAY),
            Status::Dead => {
                if self.lives == 0 {
                    d.draw_text("GAME OVER - SPACE TO RESET", 280, 400, 20, Color::RED)
                }
            }
            _ => {}
        }
        let text = format!("lives: {}", self.lives);
        d.draw_text(text.as_str(), 10, 10, 20, Color::GRAY);

        for pos in &self.dead_balls_pos {
            d.draw_circle_v(pos, self.ball.radius, Color::RAYWHITE.alpha(0.3));
        }
    }

    fn handle_collision(&mut self, dt: f32) {
        if self.ball.status != Status::Running {
            return;
        }

        for event in self.ball.update(dt) {
            match event {
                GameEvent::BallDropped => {
                    self.dead_balls_pos.push(self.ball.pos);
                    self.lives -= 1;
                    if self.lives > 0 {
                        self.ball
                            .reset(self.platform.hitbox().center_x(), self.platform.pos.y);
                        self.sounds.play_drop();
                    }
                }
                GameEvent::BallHitWall => {
                    self.sounds.play_bounce();
                }
            }
        }
        self.check_platform_collision_with_ball();
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
            self.particles.extend(Particle::spawn_particles(hit_point));
            self.sounds.play_bounce();
        }
    }

    fn handle_input(&mut self, rl: &RaylibHandle, dt: f32) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.platform.move_left(dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.platform.move_right(dt);
        }
        match self.ball.status {
            Status::Start => {
                self.place_ball_on_platform();
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    self.ball.velocity.y = -VELOCITY;
                    self.ball.status = Status::Running;
                }
            }
            Status::Dead => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.lives == 0 {
                    self.lives = MAX_LIVES;
                    self.dead_balls_pos = Vec::new();
                    self.ball
                        .reset(self.platform.hitbox().center_x(), self.platform.pos.y);
                }
            }
            Status::Running => {}
        }
    }
}

pub struct SoundManager<'a> {
    pub drop_sound: Option<Sound<'a>>,
    pub bounce_sound: Option<Sound<'a>>,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio_handle: Option<&'a RaylibAudio>) -> Self {
        match audio_handle {
            Some(h) => Self {
                drop_sound: h.new_sound("assets/dropped.wav").ok(),
                bounce_sound: h.new_sound("assets/bounce.wav").ok(),
            },
            None => Self {
                drop_sound: None,
                bounce_sound: None,
            },
        }
    }

    pub fn play_drop(&self) {
        if let Some(s) = &self.drop_sound {
            s.play();
        }
    }

    pub fn play_bounce(&self) {
        if let Some(s) = &self.bounce_sound {
            s.play();
        }
    }
}
