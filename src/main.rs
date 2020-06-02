use std::thread;
use std::time::{Duration, Instant};

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use doomfire::{DoomFire, FIRE_HEIGHT, FIRE_WIDTH, TIME_PER_FRAME};

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
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

    let mut doom_fire = DoomFire::new();

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut frame = pixels.get_frame();

            doom_fire.draw(&mut frame);
            pixels.render().expect("Failed at rendering");

            doom_fire.update();
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

            // Max the redraw to 60 fps
            let end_time = Instant::now();
            let render_time = end_time - start_time;
            if render_time < Duration::from_millis(TIME_PER_FRAME) {
                let waste_time = Duration::from_millis(TIME_PER_FRAME) - render_time;
                thread::sleep(waste_time);
            }
        }
    });
}
