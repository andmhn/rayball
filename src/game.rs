use crate::components::*;
use crate::constants::MAX_LIVES;
use crate::systems::audio::SoundManager;
use crate::systems::physics;
use crate::systems::render;
use raylib::prelude::*;

pub enum GameEvent {
    BallHitWall,
    BallDropped,
    BallHitPlatform(Vector2),
    BrickCollision(Vector2),
}

pub struct Game<'a> {
    ball: Ball,
    bricks: Vec<Brick>,
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
            bricks: Brick::generate(),
            platform: Platform::new(),
            particles: Vec::new(),
            sounds,
            lives: MAX_LIVES,
            dead_balls_pos: Vec::new(),
        };

        game.sync_ball_position();
        game
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.platform.move_left(dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.platform.move_right(dt);
        }

        self.move_ball(rl, dt);

        self.particles.iter_mut().for_each(|p| p.update(dt));
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        render::draw_world(d, &self.ball, &self.bricks, &self.platform, &self.particles);
        let won = !self.bricks.iter().any(|b| b.active);
        render::draw_game_ui(d, self.lives, &self.ball.status, &self.dead_balls_pos, won);
    }

    fn move_ball(&mut self, rl: &RaylibHandle, dt: f32) {
        match self.ball.status {
            Status::Start => {
                physics::snap_ball_to_platform(&mut self.ball, &self.platform);

                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    self.ball.launch();
                }
            }
            Status::Running => {
                let mut events = Vec::new();
                events.extend(physics::update_ball_position(&mut self.ball, dt));
                events.extend(physics::resolve_ball_collisions(
                    &mut self.ball,
                    &self.platform,
                    &mut self.bricks,
                ));
                for event in events {
                    self.handle_event(event);
                }
            }
            Status::Dead => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.lives == 0 {
                    self.reset_game();
                }
            }
            Status::Spawning => {
                if physics::transition_ball(&mut self.ball, &self.platform, dt) {
                    return; // ball is still spawning
                }
                self.ball.status = Status::Start;
                self.sync_ball_position();
            }
        }
    }

    fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::BallDropped => {
                self.dead_balls_pos.push(self.ball.pos);
                self.lives -= 1;
                if self.lives > 0 {
                    self.ball.reset();
                    self.ball.pos = render::get_ball_lives_pos(self.lives);
                    self.ball.status = Status::Spawning;
                    self.sounds.play_transition();
                }
            }
            GameEvent::BallHitWall => {
                self.sounds.play_bounce();
            }
            GameEvent::BallHitPlatform(hit_point) | GameEvent::BrickCollision(hit_point) => {
                self.particles.extend(Particle::spawn_particles(hit_point));
                self.sounds.play_bounce();
            }
        }
    }

    fn sync_ball_position(&mut self) {
        physics::snap_ball_to_platform(&mut self.ball, &self.platform);
    }

    fn reset_game(&mut self) {
        self.lives = MAX_LIVES;
        self.dead_balls_pos = Vec::new();
        self.ball.reset();
        self.sync_ball_position();
        for b in &mut self.bricks {
            b.active = true;
        }
    }
}
