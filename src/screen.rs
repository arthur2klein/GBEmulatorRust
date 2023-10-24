extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160; // Game Boy screen width
const HEIGHT: usize = 144; // Game Boy screen height
const TILE_SIZE: usize = 8; // Tile size in pixels

pub struct Screen {
    /// Buffer for the screen
    buffer: Vec<u32>,
    /// Window to draw on
    window: Window,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: vec![0; WIDTH * HEIGHT],
            window: Window::new(
                "Game Boy Graphics",
                WIDTH,
                HEIGHT,
                WindowOptions::default()
            )
                .unwrap_or_else(|e| {
                    panic!("Could not create screen: {}", e);
                }),
        }
    }

    pub fn receive_pixel(
        &mut self,
        x: u8,
        y: u8,
        c: u8
    ) {
        self.buffer[y * WIDTH + x]  = match c {
            0x01 => {
                0x555555
            },
            0x02 => {
                0xAAAAAA
            },
            0x03 => {
                0x000000
            },
            _ => {
                0xFFFFFF
            }
        }
    }

    pub fn update(&mut self) {
        window
            .update_with_buffer(&buffer)
            .unwrap_or_else(|e| {
                panic!("{}", e);
        });
    }
}
