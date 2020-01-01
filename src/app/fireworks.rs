extern crate rand;
extern crate sfml;

use rand::*;
use crate::app::*;
use sfml::graphics::*;
use sfml::system::*;

pub struct FireworksCtx<'a> {
    fireworks: Vec<Firework<'a>>,
    next_spawn: f32,
    time: f32,
}

impl FireworksCtx<'_> {
    pub fn new() -> FireworksCtx<'static> {
        FireworksCtx {
            fireworks: Vec::new(),
            next_spawn: 0.,
            time: 0.,
        }
    }
}

struct Firework<'a> {
    explode: bool,
    should_die: bool,
    position: Vector2f,
    velocity: Vector2f,
    rocket: CircleShape<'a>,
    particles: Vec<Particle<'a>>,
    lifetime: f32,
}

impl Firework<'_> {
    pub fn new() -> Firework<'static> {
        let mut firework = Firework {
            explode: false,
            should_die: false,
            position: Vector2f::new((random::<u32>() % WINDOW_WIDTH) as f32, WINDOW_HEIGHT as f32),
            velocity: Vector2f::new(0., -600.),
            rocket: CircleShape::new(8., 10),
            particles: Vec::new(),
            lifetime: 0.,
        };
        firework.rocket.set_fill_color(Color::BLACK);
        firework
    }

    pub fn position(&self) -> Vector2f {
        self.position
    }
    pub fn velocity(&self) -> Vector2f {
        self.velocity
    }

    pub fn update(&mut self, delta: f32) {
        // Keep track of lifetime
        self.lifetime += delta;
        if self.lifetime >= 5. {
            self.should_die = true;
        }

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
    radius: f32,
}

impl Particle<'_> {
    pub fn new(pos: Vector2f) -> Particle<'static> {
        let mut particle = Particle {
            velocity: Vector2f::new((random::<f32>() - 0.5) * 500., (random::<f32>() - 0.5) * 500.),
            position: pos,
            shape: CircleShape::new(3., 8),
            radius: 5.,
        };
        particle.shape.set_fill_color(Color::rgb(random(), random(), random()));
        particle
    }
    pub fn update(&mut self, delta: f32) {
        // Reduce radius
        if self.radius > 0. {
            self.radius -= 6. * delta;
        }
        // Add gravity
        self.velocity.y += 250. * delta;
        // Add velocity to position
        self.position += self.velocity * delta;
        // Set attributes
        self.shape.set_position(self.position);
        self.shape.set_radius(self.radius);
    }
    pub fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.shape);
    }
}


fn create_firework(ctx: &mut FireworksCtx) {
    ctx.fireworks.push(Firework::new());
}

pub fn update_fireworks(ctx: &mut FireworksCtx, delta: f32) {
    ctx.time += delta;
    if ctx.fireworks.len() < 12 && ctx.next_spawn <= ctx.time {
        create_firework(ctx);
        ctx.next_spawn = random::<f32>() % 5.;
        ctx.time = 0.;
    }
    for i in 0..ctx.fireworks.len() {
        let mut firework = ctx.fireworks.get_mut(i);
        if !firework.is_some() { break; }
        let mut firework = firework.unwrap();

        firework.update(delta);
        if firework.should_die == true {
            ctx.fireworks.remove(i);
        }
    }
}

pub fn draw_fireworks(ctx: &mut FireworksCtx, window: &mut RenderWindow) {
    for firework in &ctx.fireworks {
        firework.draw(window);
    }
}

