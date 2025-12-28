use rand::Rng;
use raylib::prelude::{Color, Vector2, rvec2};

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
