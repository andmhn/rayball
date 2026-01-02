use crate::components::particle::Direction;
use crate::components::*;
use crate::constants::MAX_LIVES;
use crate::systems::audio::SoundManager;
use crate::systems::physics;
use crate::systems::render;
use macroquad::prelude::*;

pub enum GameEvent {
    BallHitWall,
    BallDropped,
    BallHitPlatform(Vec2),
    BrickCollision(Vec2, Direction),
}

pub struct Game {
    ball: Ball,
    bricks: Vec<Brick>,
    platform: Platform,
    particles: Vec<Particle>,
    sounds: SoundManager,
    lives: u8,
    death_pos: Vec<Vec2>,
    won: bool,
}

impl Game {
    pub fn new(sounds: SoundManager) -> Self {
        let mut game = Self {
            ball: Ball::new(),
            bricks: Brick::generate(),
            platform: Platform::new(),
            particles: Vec::new(),
            sounds,
            lives: MAX_LIVES,
            death_pos: Vec::new(),
            won: false,
        };

        game.sync_ball_position();
        game
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.won = !self.bricks.iter().any(|b| b.active);

        if self.won && action_pressed() {
            self.reset_game();
            return;
        }

        for touch in touches() {
            if touch.position.x < screen_width() / 2. {
                self.platform.move_left(dt);
            } else {
                self.platform.move_right(dt);
            }
        }

        if is_key_down(KeyCode::Left) {
            self.platform.move_left(dt);
        }
        if is_key_down(KeyCode::Right) {
            self.platform.move_right(dt);
        }

        self.move_ball(dt);

        self.particles.iter_mut().for_each(|p| p.update(dt));
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self) {
        render::draw_world(&self.ball, &self.bricks, &self.platform, &self.particles);
        render::draw_game_ui(self.lives, &self.ball.status, &self.death_pos, self.won);
    }

    fn move_ball(&mut self, dt: f32) {
        match self.ball.status {
            Status::Start => {
                physics::snap_ball_to_platform(&mut self.ball, &self.platform);

                if action_pressed() {
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
                if action_pressed() && self.lives == 0 {
                    self.reset_game();
                }
            }
            Status::Spawning => {
                if physics::transition_ball(&mut self.ball, &self.platform, dt) {
                    return;
                }
                self.ball.status = Status::Start;
                self.sync_ball_position();
            }
        }
    }

    fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::BallDropped => {
                self.death_pos.push(self.ball.pos);
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
            GameEvent::BallHitPlatform(hit_point) => {
                self.particles.extend(Particle::spawn_particles(
                    hit_point,
                    particle::Direction::Up,
                ));
                self.sounds.play_bounce();
            }
            GameEvent::BrickCollision(hit_point, direction) => {
                self.particles
                    .extend(Particle::spawn_particles(hit_point, direction));
                self.sounds.play_bounce();
            }
        }
    }

    fn sync_ball_position(&mut self) {
        physics::snap_ball_to_platform(&mut self.ball, &self.platform);
    }

    fn reset_game(&mut self) {
        self.lives = MAX_LIVES;
        self.death_pos = Vec::new();
        self.ball.reset();
        self.platform = Platform::new();
        self.sync_ball_position();
        for b in &mut self.bricks {
            b.active = true;
        }
    }
}

fn action_pressed() -> bool {
    is_key_pressed(KeyCode::Space) || touches().iter().any(|t| t.phase == TouchPhase::Started)
}
