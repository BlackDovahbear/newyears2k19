extern crate rand;
extern crate sfml;

mod snow;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

pub static WINDOW_WIDTH: u32 = 800;
pub static WINDOW_HEIGHT: u32 = 600;

fn draw_text(window: &mut RenderWindow) {
    unimplemented!()
}

pub fn run_app() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
        "Happy New Years!",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    let mut snow_ctx = snow::SnowCtx::new();

    let mut clock = Clock::start();
    loop {
        let delta = clock.restart().as_milliseconds() as f32 / 1000.;
        // Handle events
        for event in window.poll_event() {
            if let Event::Closed = event {
                return;
            }
        }

        // Update objects
        snow::update_snow(&mut snow_ctx, delta);
        // Clear the window
        window.clear(Color::rgb(0x3e, 0x4f, 0x63));
        // Draw objects
        snow::draw_snow(&mut snow_ctx, &mut window);
        // Display things on screen
        window.display()
    }
}
