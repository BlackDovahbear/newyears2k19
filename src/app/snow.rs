extern crate rand;
extern crate sfml;

use crate::app::*;
use rand::*;
use sfml::graphics::*;
use sfml::system::*;

pub struct SnowCtx<'a> {
    snow: Vec<CircleShape<'a>>,
    initialized: bool,
}

impl SnowCtx<'_> {
    pub fn new() -> SnowCtx<'static> {
        SnowCtx {
            snow: Vec::new(),
            initialized: false,
        }
    }
}

pub fn update_snow(ctx: &mut SnowCtx, delta: f32) {
    if !ctx.initialized {
        init_snow(ctx);
    }
    for mut flake in &mut ctx.snow {
        flake.move_((20. * delta, 50. * delta));
        clamp_position(&mut flake);
    }
}

pub fn draw_snow(ctx: &mut SnowCtx, window: &mut RenderWindow) {
    for flake in &ctx.snow {
        window.draw(flake);
    }
}

fn init_snow(ctx: &mut SnowCtx) {
    for _i in 1..50 {
        ctx.snow.push(CircleShape::new(2., 5));
        let flake = ctx.snow.last_mut().unwrap();

        let pos: Vector2f = Vector2f::new(
            (random::<u32>() % WINDOW_WIDTH) as f32,
            (random::<u32>() % WINDOW_HEIGHT) as f32,
        );

        flake.set_position(pos);
        flake.set_fill_color(Color::WHITE);
    }
    ctx.initialized = true;
}

fn clamp_position(flake: &mut CircleShape) {
    if flake.position().x > WINDOW_WIDTH as f32 {
        flake.move_((-(WINDOW_WIDTH as f32), 0.));
    }
    if flake.position().y > WINDOW_HEIGHT as f32 {
        flake.move_((0., -(WINDOW_HEIGHT as f32)));
    }
}
