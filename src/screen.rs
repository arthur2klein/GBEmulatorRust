extern crate minifb;

use minifb::{Window, WindowOptions};

const WIDTH: u8 = 160; // Game Boy screen width
const HEIGHT: u8 = 144; // Game Boy screen height

pub struct Screen {
    /// Buffer for the screen
    buffer: Vec<u32>,
    /// Window to draw on
    window: Window,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: vec![0; WIDTH as usize * HEIGHT as usize],
            window: Window::new(
                "Game Boy Graphics",
                WIDTH as usize,
                HEIGHT as usize,
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
        self.buffer[y as usize * WIDTH as usize + x as usize]  = match c {
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
        self.window
            .update_with_buffer_size(
                &self.buffer,
                WIDTH as usize,
                HEIGHT as usize
            ).unwrap_or_else(|e| {
                panic!("{}", e);
        });
    }
}
