#![allow(dead_code)]
#[macro_use]

extern crate sfml;

use sfml::graphics::{Sprite, Texture, Font, Text, Color, RenderTarget, RenderWindow};
use sfml::window::{Key, VideoMode, Event, window_style};
use sfml::graphics::Transformable;

#[derive(Clone, Copy)]
pub struct TriangleShape;

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new_init(800, 600, 32),
        "Tiny Taco SFML",
        window_style::CLOSE,
        &Default::default()
    ).unwrap();

    // Set the window vsync
    window.set_vertical_sync_enabled(true);

    // Draw some text
    let taco_font = Font::new_from_file("resources/sansation.ttf").unwrap();
    let taco_text = Text::new_init("Tiny Taco Team", &taco_font, 50).unwrap();

    // Load a taco texture
    let taco_texture = Texture::new_from_file("resources/taco.png").unwrap();

    // Create a sprite
    let mut taco_sprite = Sprite::new().unwrap();

    // Set the sprite to use the texture
    taco_sprite.set_texture(&taco_texture, true);

    // Set position and scale
    taco_sprite.set_position2f(50f32, 100f32);
    taco_sprite.set_scale2f(0.2f32, 0.2f32);

    loop {
        for event in window.events() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());

        // Draw the taco sprite
        window.draw(&taco_sprite);

        // Draw the taco text
        window.draw(&taco_text);

        window.display();

    }
}
