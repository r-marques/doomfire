mod utils;

use doomfire::{DoomFire, FIRE_HEIGHT, FIRE_WIDTH};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct DoomFireWasm {
    doom_fire: DoomFire,
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

/// This a wrapper around the DoomFire library.
///
/// The main reason for doing this is so that we can keep the buffer on the
/// webassembly side and avoid necessary copies between webassembly memory and
/// the javascript heap.
///
/// This is a recommended optimization
/// https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript
#[wasm_bindgen]
impl DoomFireWasm {
    pub fn new() -> Self {
        DoomFireWasm {
            doom_fire: DoomFire::new(),
            // 4 bytes per pixel
            buffer: vec![0; FIRE_HEIGHT * FIRE_WIDTH * 4],
            width: FIRE_WIDTH as u32,
            height: FIRE_HEIGHT as u32,
        }
    }

    pub fn update(&mut self) {
        self.doom_fire.update();
    }

    pub fn draw(&mut self) {
        self.doom_fire.draw(&mut self.buffer);
    }

    pub fn get_buffer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
