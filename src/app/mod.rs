extern crate sfml;
extern crate rand;

use rand::*;
use sfml::system::*;
use sfml::window::*;
use sfml::graphics::*;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

pub fn draw_snow(snow: &mut Vec<CircleShape>, window: &mut RenderWindow, delta: f32) {
    for flake in snow {
        flake.move_((5. * delta, 10. * delta));
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
    for _i in 1..10 {
        snow.push(CircleShape::new(2., 5));
        let flake = snow.last_mut().unwrap();
        flake.set_position(((random::<u32>() % WINDOW_WIDTH) as f32, 0.));
        flake.set_fill_color(Color::WHITE);
        let pos = flake.position();
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
