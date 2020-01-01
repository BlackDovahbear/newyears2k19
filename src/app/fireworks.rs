extern crate rand;
extern crate sfml;

use rand::*;
use crate::app::*;
use sfml::graphics::*;
use sfml::system::*;

pub struct FireworksCtx<'a> {
    fireworks: Vec<Firework<'a>>
}

impl FireworksCtx<'_> {
    pub fn new() -> FireworksCtx<'static> {
        FireworksCtx {
            fireworks: Vec::new(),
        }
    }
}

struct Firework<'a> {
    explode: bool,
    position: Vector2f,
    velocity: Vector2f,
    rocket: CircleShape<'a>,
    particles: Vec<Particle<'a>>,
}

impl Firework<'_> {
    pub fn new() -> Firework<'static> {
        Firework {
            explode: false,
            position: Vector2f::new((random::<u32>() % WINDOW_WIDTH) as f32, WINDOW_HEIGHT as f32),
            velocity: Vector2f::new(0., -600.),
            rocket: CircleShape::new(5., 10),
            particles: Vec::new(),
        }
    }

    pub fn position(&self) -> Vector2f {
        self.position
    }
    pub fn velocity(&self) -> Vector2f {
        self.velocity
    }

    pub fn update(&mut self, delta: f32) {
        if !self.explode {
            // Add gravity
            self.velocity.y += 500. * delta;
            // Update position using velocity
            self.position.x += self.velocity.x * delta;
            self.position.y += self.velocity.y * delta;
            // Set position
            self.rocket.set_position(self.position);
            // Explode when vertical velocity is zero
            if self.velocity.y >= 0. {
                self.explode = true;
                for i in 1..50 { self.particles.push(Particle::new(self.position)); }
            }
        } else {
            for part in &mut self.particles {
                part.update(delta);
            }
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        if !self.explode {
            window.draw(&self.rocket);
        } else {
            for part in &self.particles {
                part.draw(window);
            }
        }
    }
}

struct Particle<'a> {
    velocity: Vector2f,
    position: Vector2f,
    shape: CircleShape<'a>,
}

impl Particle<'_> {
    pub fn new(pos: Vector2f) -> Particle<'static> {
        Particle {
            velocity: Vector2f::new((random::<f32>() - 0.5) * 500., (random::<f32>() - 0.5) * 500.),
            position: pos,
            shape: CircleShape::new(2., 5),
        }
    }
    pub fn update(&mut self, delta: f32) {
        // Add gravity
        self.velocity.y += 250. * delta;
        // Add velocity to position
        self.position += self.velocity * delta;
        // Set position
        self.shape.set_position(self.position);
    }
    pub fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.shape);
    }
}


fn create_firework(ctx: &mut FireworksCtx) {
    ctx.fireworks.push(Firework::new());
}

pub fn update_fireworks(ctx: &mut FireworksCtx, delta: f32) {
    if ctx.fireworks.len() <= 0 && random() {
        create_firework(ctx);
    }
    for firework in &mut ctx.fireworks {
        firework.update(delta);
    }
}

pub fn draw_fireworks(ctx: &mut FireworksCtx, window: &mut RenderWindow) {
    for firework in &ctx.fireworks {
        firework.draw(window);
    }
}

