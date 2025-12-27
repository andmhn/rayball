use crate::constants::*;
use crate::physics::HitBox;
use rand::Rng;
use raylib::prelude::*;

#[derive(PartialEq)]
pub enum Status {
    Start,
    Running,
    Dead,
}

pub struct Ball {
    pub pos: Vector2,
    pub velocity: Vector2,
    pub status: Status,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vector2::zero(),
            velocity: Vector2::zero(),
            status: Status::Start,
            radius: BALL_RADIUS,
        }
    }

    pub fn pause(&mut self) {
        self.velocity = Vector2::zero();
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
        self.handle_wall_collisions();
    }

    fn handle_wall_collisions(&mut self) {
        let touched_down = (self.pos.y + self.radius >= WINDOW_H) && (self.velocity.y > 0.0);
        if touched_down {
            self.pos.y = WINDOW_H - self.radius;
            self.velocity.y *= -1.0;
            self.status = Status::Dead;
        }
        let touched_up = self.pos.y < self.radius && self.velocity.y < 0.0;
        if touched_up {
            self.pos.y = self.radius;
            self.velocity.y *= -1.0;
        }
        let touched_right = self.pos.x + self.radius >= WINDOW_W && self.velocity.x > 0.0;
        if touched_right {
            self.pos.x = WINDOW_W - self.radius;
            self.velocity.x *= -1.0;
        }
        let touched_left = self.pos.x < self.radius && self.velocity.x < 0.0;
        if touched_left {
            self.pos.x = self.radius;
            self.velocity.x *= -1.0;
        }
    }

    pub fn collides_with_hitbox(&self, hitbox: &HitBox) -> bool {
        hitbox.overlaps_circle(self.pos, BALL_RADIUS)
    }

    pub fn is_dying(&self) -> bool {
        (self.status == Status::Dead) && (self.velocity.y < 0.)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let color = if self.status == Status::Dead {
            Color::YELLOW.alpha(0.5)
        } else {
            Color::YELLOW
        };
        d.draw_circle_v(self.pos, self.radius, color);
    }
}

pub struct Platform {
    pub pos: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub fn new() -> Self {
        let width = PLATFORM_W;
        let height = PLATFORM_H;
        let pos = Vector2 {
            x: (WINDOW_W - width) / 2.,
            y: WINDOW_H - height,
        };
        Platform { pos, width, height }
    }

    pub fn move_left(&mut self, dt: f32) {
        self.pos.x -= 1000. * dt;
        if self.pos.x < 0. {
            self.pos.x = 0.;
        }
    }

    pub fn move_right(&mut self, dt: f32) {
        self.pos.x += 1000. * dt;
        if self.pos.x + self.width > WINDOW_W {
            self.pos.x = WINDOW_W - self.width;
        }
    }

    pub fn hitbox(&self) -> HitBox {
        HitBox::new(self.pos, self.width, self.height)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.pos, rvec2(self.width, self.height), Color::RAYWHITE);
    }
}

pub struct Particle {
    pub pos: Vector2,
    pub vel: Vector2,
    pub life: f32,
    pub color: Color,
}

impl Particle {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.life -= dt * 2.0; // Fade with time
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, 2., self.color.alpha(self.life));
    }

    pub fn spawn_particles(origin: Vector2) -> Vec<Particle> {
        let mut particles = Vec::new();
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
            particles.push(particle);
        }
        particles
    }
}
