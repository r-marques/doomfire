use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

use doomfire::{DoomFire, FIRE_HEIGHT, FIRE_WIDTH, TIME_PER_FRAME};

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut window = Window::new(
        "Doom Fire - ESC to exit",
        FIRE_WIDTH,
        FIRE_HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(Duration::from_millis(TIME_PER_FRAME)));

    let mut doom_fire = DoomFire::new();

    let mut frame: Vec<u8> = vec![0; FIRE_WIDTH * FIRE_HEIGHT * 4];
    let mut buffer: Vec<u32> = vec![0; FIRE_WIDTH * FIRE_HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        doom_fire.draw(&mut frame);

        // DoomFire expects a &[u8] to write the pixels with a RGBA encoding but minifb
        // expects a &[u32] with a 0RGB pixel encoding, where the upper 8 bits are ignored.
        for (i, pixel) in frame.chunks_exact(4).enumerate() {
            buffer[i] = from_u8_rgb(pixel[0], pixel[1], pixel[2]);
        }

        window
            .update_with_buffer(&buffer, FIRE_WIDTH, FIRE_HEIGHT)
            .expect("Failed to update with buffer");
        doom_fire.update();
    }
}
