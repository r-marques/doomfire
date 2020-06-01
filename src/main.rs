use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const FIRE_WIDTH: usize = 320;
const FIRE_HEIGHT: usize = 168;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let pallet = [
        from_u8_rgb(0x07, 0x07, 0x07),
        from_u8_rgb(0x1F, 0x07, 0x07),
        from_u8_rgb(0x2F, 0x0F, 0x07),
        from_u8_rgb(0x47, 0x0F, 0x07),
        from_u8_rgb(0x57, 0x17, 0x07),
        from_u8_rgb(0x67, 0x1F, 0x07),
        from_u8_rgb(0x77, 0x1F, 0x07),
        from_u8_rgb(0x8F, 0x27, 0x07),
        from_u8_rgb(0x9F, 0x2F, 0x07),
        from_u8_rgb(0xAF, 0x3F, 0x07),
        from_u8_rgb(0xBF, 0x47, 0x07),
        from_u8_rgb(0xC7, 0x47, 0x07),
        from_u8_rgb(0xDF, 0x4F, 0x07),
        from_u8_rgb(0xDF, 0x57, 0x07),
        from_u8_rgb(0xDF, 0x57, 0x07),
        from_u8_rgb(0xD7, 0x5F, 0x07),
        from_u8_rgb(0xD7, 0x5F, 0x07),
        from_u8_rgb(0xD7, 0x67, 0x0F),
        from_u8_rgb(0xCF, 0x6F, 0x0F),
        from_u8_rgb(0xCF, 0x77, 0x0F),
        from_u8_rgb(0xCF, 0x7F, 0x0F),
        from_u8_rgb(0xCF, 0x87, 0x17),
        from_u8_rgb(0xC7, 0x87, 0x17),
        from_u8_rgb(0xC7, 0x8F, 0x17),
        from_u8_rgb(0xC7, 0x97, 0x1F),
        from_u8_rgb(0xBF, 0x9F, 0x1F),
        from_u8_rgb(0xBF, 0x9F, 0x1F),
        from_u8_rgb(0xBF, 0xA7, 0x27),
        from_u8_rgb(0xBF, 0xA7, 0x27),
        from_u8_rgb(0xBF, 0xAF, 0x2F),
        from_u8_rgb(0xB7, 0xAF, 0x2F),
        from_u8_rgb(0xB7, 0xB7, 0x2F),
        from_u8_rgb(0xB7, 0xB7, 0x37),
        from_u8_rgb(0xCF, 0xCF, 0x6F),
        from_u8_rgb(0xDF, 0xDF, 0x9F),
        from_u8_rgb(0xEF, 0xEF, 0xC7),
        from_u8_rgb(0xFF, 0xFF, 0xFF),
    ];

    let mut rng = rand::thread_rng();

    let mut window = Window::new(
        "Test - ESC to exit",
        FIRE_WIDTH,
        FIRE_HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    // set the whole screen to color rgbs[0]
    let mut fire_pixels: Vec<usize> = vec![0; FIRE_WIDTH * FIRE_HEIGHT];
    let mut buffer: Vec<u32> = vec![0; FIRE_WIDTH * FIRE_HEIGHT];
    // set the bottom line to color rgbs[37]
    for i in 0..FIRE_WIDTH {
        fire_pixels[(FIRE_HEIGHT - 1) * FIRE_WIDTH + i] = 36;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..FIRE_WIDTH {
            for y in 1..FIRE_HEIGHT {
                let src = y * FIRE_WIDTH + x;
                let pixel = fire_pixels[src];

                if pixel == 0 {
                    fire_pixels[src - FIRE_WIDTH] = 0;
                } else {
                    let rand_idx = (rng.gen_range(0.0, 3.0) + 0.5) as usize & 3;
                    let dst = src - rand_idx + 1;
                    fire_pixels[dst - FIRE_WIDTH] = pixel - (rand_idx & 1);
                }
            }
        }

        for i in 0..(FIRE_WIDTH * FIRE_HEIGHT) {
            buffer[i] = pallet[fire_pixels[i]];
        }

        window
            .update_with_buffer(&buffer, FIRE_WIDTH, FIRE_HEIGHT)
            .expect("Failed to update with buffer")
    }
}
