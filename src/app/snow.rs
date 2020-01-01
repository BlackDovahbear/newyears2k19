extern crate rand;
extern crate sfml;

use crate::app::*;
use rand::*;
use sfml::graphics::*;
use sfml::system::*;

pub struct SnowCtx<'a> {
    snow: Vec<CircleShape<'a>>,
    initialized: bool,
    counter: i32,
    offsets: Vec<i32>,
}

impl SnowCtx<'_> {
    pub fn new() -> SnowCtx<'static> {
        SnowCtx {
            snow: Vec::new(),
            offsets: Vec::new(),
            initialized: false,
            counter: 0,
        }
    }
}

pub fn update_snow(ctx: &mut SnowCtx, delta: f32) {
    ctx.counter = ctx.counter + 1;
    if !ctx.initialized {
        init_snow(ctx);
    }
    for i in 0..ctx.snow.len() {
        let mut flake = ctx.snow.get_mut(i).unwrap();
        let iter = ctx.counter + ctx.offsets.get(i).unwrap();
        let sin = f32::sin(iter as f32 * 5. * delta) * 20.;
        flake.move_(((20. + sin) * delta, 50. * delta));
        clamp_position(&mut flake);
    }
}

pub fn draw_snow(ctx: &mut SnowCtx, window: &mut RenderWindow) {
    for flake in &ctx.snow {
        window.draw(flake);
    }
}

fn init_snow(ctx: &mut SnowCtx) {
    for _i in 1..100 {
        ctx.snow.push(CircleShape::new(2., 5));
        let flake = ctx.snow.last_mut().unwrap();

        let pos: Vector2f = Vector2f::new(
            (random::<u32>() % WINDOW_WIDTH) as f32,
            (random::<u32>() % WINDOW_HEIGHT) as f32,
        );

        flake.set_position(pos);
        flake.set_fill_color(Color::WHITE);
        ctx.offsets.push(random());
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
