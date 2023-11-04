extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use crate::state::key_state::KeyState;

/// Game Boy screen width
const WIDTH: u8 = 160; 
/// Game Boy screen height
const HEIGHT: u8 = 144;
/// Scale of the window of the emulator
const PIXEL_SIZE: usize = 5;

/// Creates a window for the emulator
pub struct Screen {
    /// Buffer for the screen
    buffer: Vec<u32>,
    /// Window to draw on
    window: Window,
    /// State of the key presses
    pub key_state: KeyState,
}

impl Screen {
    /// Create a new window
    ///
    /// # Returns
    /// **Screen**: Screen that can be used by the emulator
    pub fn new() -> Screen {
        let mut res = Screen {
            buffer: vec![
                0xFFFFFF;
                PIXEL_SIZE * WIDTH as usize * PIXEL_SIZE * HEIGHT as usize
            ],
            window: Window::new(
                "Game Boy Graphics",
                PIXEL_SIZE * WIDTH as usize,
                PIXEL_SIZE * HEIGHT as usize,
                WindowOptions::default()
                )
                .unwrap_or_else(|e| {
                    panic!("Could not create screen: {}", e);
                }
            ),
            key_state: KeyState::new(),
        };
        res.update();
        res
    }

    /// Verify what button is being pushed
    ///
    /// # Returns
    /// **bool**: Is the escape key being pressed
    pub fn update_key_press(&mut self) -> bool {
        self.update();
        if !self.window.is_active() {
            println!("WINDOW IS NOT ACTIVE");
        }
        self.key_state.update(
            self.window.is_key_down(Key::Space),
            self.window.is_key_down(Key::S),
            self.window.is_key_down(Key::D),
            self.window.is_key_down(Key::F),
            self.window.is_key_down(Key::Up),
            self.window.is_key_down(Key::Down),
            self.window.is_key_down(Key::Right),
            self.window.is_key_down(Key::Left),
        );
        self.window.is_key_down(Key::Escape)
    }

    /// Change the color of a pixel of the GameBoy
    ///
    /// # Arguments
    /// **x (u8)**: x coordinate of the object
    /// **y (u8)**: y coordinate of the object
    /// **c (u8)**: Color of the pixel (00 to 11 for white to black)
    pub fn receive_pixel(
        &mut self,
        x: u8,
        y: u8,
        c: u8
    ) {
        for i in 0..PIXEL_SIZE {
            for j in 0..PIXEL_SIZE {
                self.buffer[
                    i + PIXEL_SIZE * y as usize * WIDTH as usize +
                    j + PIXEL_SIZE * x as usize
                ]  = match c {
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
        }
    }

    /// Refresh the screen
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
