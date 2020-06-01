use std::thread;
use std::time::{Duration, Instant};

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use rand::Rng;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const FIRE_WIDTH: usize = 320;
const FIRE_HEIGHT: usize = 168;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let pallet = [
        [0x07, 0x07, 0x07, 0xFF],
        [0x1F, 0x07, 0x07, 0xFF],
        [0x2F, 0x0F, 0x07, 0xFF],
        [0x47, 0x0F, 0x07, 0xFF],
        [0x57, 0x17, 0x07, 0xFF],
        [0x67, 0x1F, 0x07, 0xFF],
        [0x77, 0x1F, 0x07, 0xFF],
        [0x8F, 0x27, 0x07, 0xFF],
        [0x9F, 0x2F, 0x07, 0xFF],
        [0xAF, 0x3F, 0x07, 0xFF],
        [0xBF, 0x47, 0x07, 0xFF],
        [0xC7, 0x47, 0x07, 0xFF],
        [0xDF, 0x4F, 0x07, 0xFF],
        [0xDF, 0x57, 0x07, 0xFF],
        [0xDF, 0x57, 0x07, 0xFF],
        [0xD7, 0x5F, 0x07, 0xFF],
        [0xD7, 0x5F, 0x07, 0xFF],
        [0xD7, 0x67, 0x0F, 0xFF],
        [0xCF, 0x6F, 0x0F, 0xFF],
        [0xCF, 0x77, 0x0F, 0xFF],
        [0xCF, 0x7F, 0x0F, 0xFF],
        [0xCF, 0x87, 0x17, 0xFF],
        [0xC7, 0x87, 0x17, 0xFF],
        [0xC7, 0x8F, 0x17, 0xFF],
        [0xC7, 0x97, 0x1F, 0xFF],
        [0xBF, 0x9F, 0x1F, 0xFF],
        [0xBF, 0x9F, 0x1F, 0xFF],
        [0xBF, 0xA7, 0x27, 0xFF],
        [0xBF, 0xA7, 0x27, 0xFF],
        [0xBF, 0xAF, 0x2F, 0xFF],
        [0xB7, 0xAF, 0x2F, 0xFF],
        [0xB7, 0xB7, 0x2F, 0xFF],
        [0xB7, 0xB7, 0x37, 0xFF],
        [0xCF, 0xCF, 0x6F, 0xFF],
        [0xDF, 0xDF, 0x9F, 0xFF],
        [0xEF, 0xEF, 0xC7, 0xFF],
        [0xFF, 0xFF, 0xFF, 0xFF],
    ];

    let mut rng = rand::thread_rng();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(FIRE_WIDTH as f64, FIRE_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Doom Fire")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Failed to build window")
    };
    let mut hidpi_factor = window.scale_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(FIRE_WIDTH as u32, FIRE_HEIGHT as u32, surface);
        Pixels::new(FIRE_WIDTH as u32, FIRE_HEIGHT as u32, surface_texture)
            .expect("Failed to create Pixels")
    };

    // set the whole screen to color rgbs[0]
    let mut fire_pixels: Vec<usize> = vec![0; FIRE_WIDTH * FIRE_HEIGHT];
    // set the bottom line to color rgbs[37]
    for i in 0..FIRE_WIDTH {
        fire_pixels[(FIRE_HEIGHT - 1) * FIRE_WIDTH + i] = 36;
    }

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
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

            let frame = pixels.get_frame();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                pixel.copy_from_slice(&pallet[fire_pixels[i]]);
            }
            pixels.render().expect("Failed at rendering");
        }

        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // request redraw
            window.request_redraw();

            let end_time = Instant::now();
            let render_time = end_time - start_time;
            if render_time < Duration::from_millis(16) {
                let waste_time = Duration::from_millis(16) - render_time;
                thread::sleep(waste_time);
            }
        }
    });
}
