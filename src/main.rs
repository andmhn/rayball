use raylib::prelude::*;

const WINDOW_W: f32 = 900.;
const WINDOW_H: f32 = 600.;
const BALL_RADIUS: f32 = 20.0;
const VELOCITY: f32 = 900.0;

struct Ball {
    pos: Vector2,
    velocity: Vector2,
    dead: bool,
}

impl Ball {
    fn new() -> Self {
        Ball {
            pos: Vector2 {
                x: WINDOW_W / 2.,
                y: WINDOW_H / 2.,
            },
            velocity: Vector2 {
                x: 0.0,
                y: VELOCITY,
            },
            dead: false,
        }
    }

    fn pause(&mut self) {
        self.velocity.x = 0.;
        self.velocity.y = 0.;
    }

    fn restart(&mut self, pos: Vector2) {
        self.dead = false;
        self.pos = pos;
        self.velocity = Vector2 {
            x: 0.0,
            y: VELOCITY,
        };
    }

    fn update(&mut self, dt: f32) {
        self.pos.x += self.velocity.x * dt;
        self.pos.y += self.velocity.y * dt;

        let touched_down = (self.pos.y + BALL_RADIUS >= WINDOW_H) && (self.velocity.y > 0.0);
        if touched_down {
            self.pos.y = WINDOW_H - BALL_RADIUS;
            self.velocity.y *= -1.0;
            self.dead = true;
        }
        let touched_up = self.pos.y < BALL_RADIUS && self.velocity.y < 0.0;
        if touched_up {
            self.pos.y = BALL_RADIUS;
            self.velocity.y *= -1.0;
        }
        let touched_right = self.pos.x + BALL_RADIUS >= WINDOW_W && self.velocity.x > 0.0;
        if touched_right {
            self.pos.x = WINDOW_W - BALL_RADIUS;
            self.velocity.x *= -1.0;
        }
        let touched_left = self.pos.x < BALL_RADIUS && self.velocity.x < 0.0;
        if touched_left {
            self.pos.x = BALL_RADIUS;
            self.velocity.x *= -1.0;
        }
    }

    fn check_falling(&self) -> bool {
        self.dead && self.velocity.y < 0.
    }
}

const PLATFORM_W: f32 = 300.;
const PLATFORM_H: f32 = 10.;

struct Platform {
    pos: Vector2,
}

impl Platform {
    fn new() -> Self {
        Platform {
            pos: Vector2 {
                x: (WINDOW_W - PLATFORM_W) / 2.,
                y: WINDOW_H - PLATFORM_H,
            },
        }
    }

    fn move_left(&mut self, dt: f32) {
        self.pos.x -= 1000. * dt;
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }
    }

    fn move_right(&mut self, dt: f32) {
        self.pos.x += 1000. * dt;
        if self.pos.x + PLATFORM_W > WINDOW_W {
            self.pos.x = WINDOW_W - PLATFORM_W;
        }
    }
}

struct Game<'a> {
    ball: Ball,
    platform: Platform,
    audio_sample: Option<Sound<'a>>,
}

impl<'a> Game<'a> {
    fn new(audio_handle: &'a RaylibAudio) -> Self {
        let path = "assets/dropped.wav";
        let sound = audio_handle.new_sound(path).ok();

        Self {
            ball: Ball::new(),
            platform: Platform::new(),
            audio_sample: sound,
        }
    }

    // TODO: create a helper BOX wrapper with pos and size
    fn check_platform_collision_with_ball(&mut self) {
        let x = (self.ball.pos.x + BALL_RADIUS > self.platform.pos.x)
            && (self.ball.pos.x - BALL_RADIUS < self.platform.pos.x + PLATFORM_W);
        let y = (self.ball.pos.y + BALL_RADIUS >= WINDOW_H - PLATFORM_H)
            && (self.ball.velocity.y > 0.0);

        if x && y {
            self.ball.pos.y = WINDOW_H - (BALL_RADIUS + PLATFORM_H);
            self.ball.velocity.y *= -1.0;

            // difference of ball's x from the center of platform
            let platform_center = self.platform.pos.x + (PLATFORM_W / 2.);
            let diff: f32 = self.ball.pos.x - platform_center;

            self.ball.velocity.x = (diff / (PLATFORM_W / 2.0)) * VELOCITY;
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.ball.dead {
            self.ball.restart(Vector2 {
                x: WINDOW_W / 2.,
                y: WINDOW_H / 2.,
            });
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.platform.move_left(dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.platform.move_right(dt);
        }

        self.ball.update(dt);

        if self.ball.check_falling() {
            if let Some(s) = &self.audio_sample {
                s.play();
                self.ball.pause();
            }
        }
        self.check_platform_collision_with_ball();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.platform.pos,
            Vector2 {
                x: PLATFORM_W,
                y: PLATFORM_H,
            },
            Color::RAYWHITE,
        );
        d.draw_circle_v(self.ball.pos, BALL_RADIUS, Color::YELLOW);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_W as i32, WINDOW_H as i32)
        .title("rayball")
        .build();

    let audio = RaylibAudio::init_audio_device();

    let mut game = match &audio {
        Ok(audio_handle) => Game::new(audio_handle),
        Err(e) => {
            println!("Warning: Audio failed to initialize: {e}. Playing in silent mode.");
            Game {
                ball: Ball::new(),
                platform: Platform::new(),
                audio_sample: None,
            }
        }
    };

    let bg_color = Color::new(23, 25, 29, 255);
    rl.set_target_fps(120);

    while !rl.window_should_close() {
        game.update(&rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);
        game.draw(&mut d);
    }
}
