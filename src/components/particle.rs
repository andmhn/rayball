use macroquad::prelude::*;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub color: Color,
}

impl Particle {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.life -= dt * 2.0; // Fade with time
    }

    pub fn spawn_particles(origin: Vec2, direction: Direction) -> Vec<Particle> {
        let mut particles = Vec::new();

        for _ in 0..15 {

            let positive = rand::gen_range(-200.0, 200.0);
            let negative = rand::gen_range(-400.0, -100.0);
            let vel_up = Vec2::new(positive, negative);
            let vel_left = Vec2::new(negative, positive);
            let vel: Vec2;

            match direction {
                Direction::Up => vel = vel_up,
                Direction::Down => vel = vel_up * -1.,
                Direction::Left => vel = vel_left,
                Direction::Right => vel = vel_left * -1.,
            }

            let particle = Particle {
                color: WHITE,
                life: 1.0,
                pos: origin,
                vel,
            };
            particles.push(particle);
        }
        particles
    }
}
