extern crate rand;
extern crate sfml;

mod fireworks;
mod snow;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

pub static WINDOW_WIDTH: u32 = 800;
pub static WINDOW_HEIGHT: u32 = 600;

fn load_font() -> SfBox<Font> {
    let font = Font::from_file("res/LiberationMono-Regular.ttf");
    if font.is_none() {
        println!("Error: Could not find font in \"../res/\" folder.");
    }
    font.unwrap()
}

fn rainbow_text(text: &mut Text, t: f32) {
    let r = f32::sin(t + 1.5) * 127. + 127.;
    let g = f32::sin(t - 0.5) * 127. + 127.;
    let b = f32::sin(t + 3.5) * 127. + 127.;
    text.set_fill_color(Color::rgb(r as u8, g as u8, b as u8));
}

fn create_text<'a>(string: &str, font: &'a Font) -> Text<'a> {
    let mut text = Text::new(string, font, 56);
    // Set text to center of screen
    let bounds = text.local_bounds();
    text.set_position((WINDOW_WIDTH as f32 / 2., WINDOW_HEIGHT as f32 / 2.));
    text.move_((-bounds.width / 2., -bounds.height / 2.));
    // Set appearance
    text.set_fill_color(Color::rgb(0x23, 0x4a, 0x1e));
    text.set_outline_color(Color::WHITE);
    text.set_outline_thickness(1.);
    text
}

pub fn run_app() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
        "Happy New Years!",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);

    let font = load_font();
    let mut text = create_text("Happy New Years 2019", &*font);

    let mut snow_ctx = snow::SnowCtx::new();
    let mut fireworks_ctx = fireworks::FireworksCtx::new();

    let mut clock = Clock::start();
    let mut t: i32 = 0;
    loop {
        let delta = clock.restart().as_milliseconds() as f32 / 1000.;
        // Handle events
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                return;
            }
        }

        t += 1;
        rainbow_text(&mut text, t as f32 * delta);

        // Update objects
        snow::update_snow(&mut snow_ctx, delta);
        fireworks::update_fireworks(&mut fireworks_ctx, delta);
        // Clear the window
        window.clear(Color::rgb(0, 10, 20));
        // Draw objects
        snow::draw_snow(&mut snow_ctx, &mut window);
        fireworks::draw_fireworks(&mut fireworks_ctx, &mut window);

        // Draw text
        window.draw(&text);

        // Display things on screen
        window.display()
    }
}
