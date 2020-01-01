extern crate sfml;
extern crate rand;

use rand::*;
use sfml::system::*;
use sfml::window::*;
use sfml::graphics::*;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

fn clamp_position(flake: &mut CircleShape) {
    if flake.position().x > WINDOW_WIDTH as f32 {
        flake.move_((-(WINDOW_WIDTH as f32), 0.));
    }
    if flake.position().y > WINDOW_HEIGHT as f32 {
        flake.move_((0., -(WINDOW_HEIGHT as f32)));
    }
}

fn draw_snow(snow: &mut Vec<CircleShape>, window: &mut RenderWindow, delta: f32) {
    for flake in snow {
        flake.move_((20. * delta, 50. * delta));
        clamp_position(flake);
        window.draw(flake);
    }
}

pub fn run_app() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
        "Happy New Years!",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    // Create objects
    let mut snow: Vec<CircleShape> = Vec::new();
    for _i in 1..50 {
        snow.push(CircleShape::new(2., 5));
        let flake = snow.last_mut().unwrap();

        let pos: Vector2f = Vector2f::new(
            (random::<u32>() % WINDOW_WIDTH) as f32,
            (random::<u32>() % WINDOW_HEIGHT) as f32,
        );

        flake.set_position(pos);
        flake.set_fill_color(Color::WHITE);
    }

    let mut clock = Clock::start();
    loop {
        let delta = clock.restart().as_milliseconds() as f32 / 1000.;
        // Handle events
        for event in window.poll_event() {
            if let Event::Closed = event {
                return;
            }
        }

        // Clear the window
        window.clear(Color::rgb(0x3e, 0x4f, 0x63));

        draw_snow(&mut snow, &mut window, delta);

        // Display things on screen
        window.display()
    }
}
