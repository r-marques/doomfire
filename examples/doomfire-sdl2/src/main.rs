use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use doomfire::{DoomFire, FIRE_HEIGHT, FIRE_WIDTH};

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize sdl2 context");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    let window = video_subsystem
        .window(
            "Doom Fire - ESC to exit",
            FIRE_WIDTH as u32,
            FIRE_HEIGHT as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .expect("Failed to initialize window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to initialize canvas");

    let texture_creator = canvas.texture_creator();

    let mut fire_texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            FIRE_WIDTH as u32,
            FIRE_HEIGHT as u32,
        )
        .expect("Failed to create texture");

    let mut rect = Rect::new(0, 0, FIRE_WIDTH as u32, FIRE_HEIGHT as u32);

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to setup event pump");

    let mut doom_fire = DoomFire::new();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event: WindowEvent::Resized(width, height),
                } => rect.resize(width as u32, height as u32),
                _ => (),
            }
        }

        fire_texture
            .with_lock(None, |mut buffer: &mut [u8], _| doom_fire.draw(&mut buffer))
            .expect("Failed to update texture");

        canvas
            .copy(&fire_texture, None, Some(rect))
            .expect("Failed to copy texture to canvas");
        canvas.present();

        doom_fire.update();
    }
}
