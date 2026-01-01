use macroquad::prelude::*;

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

    pub fn spawn_particles(origin: Vec2) -> Vec<Particle> {
        let mut particles = Vec::new();

        for _ in 0..15 {
            let particle = Particle {
                color: WHITE,
                life: 1.0,
                pos: origin,
                vel: Vec2::new(
                    rand::gen_range(-200.0, 200.0),
                    rand::gen_range(-400.0, -100.0),
                ),
            };
            particles.push(particle);
        }
        particles
    }
}
