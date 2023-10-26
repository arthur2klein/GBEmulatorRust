extern crate minifb;

use minifb::{Key, Window, WindowOptions};
/// Game Boy screen width
const WIDTH: u8 = 160; 
/// Game Boy screen height
const HEIGHT: u8 = 144;
/// Scale of the window of the emulator
const PIXEL_SIZE: usize = 5;

#[derive(Debug)]
/// Contains information about what key is being pushed
pub struct KeyState {
    /// Is the start key pressed
    pub is_start_pressed: bool,
    /// Is the select key pressed
    pub is_select_pressed: bool,
    /// Is the A key pressed
    pub is_a_pressed: bool,
    /// Is the B key pressed
    pub is_b_pressed: bool,
    /// Is the Up Arrow pressed
    pub is_up_pressed: bool,
    /// Is the Down Arrow pressed
    pub is_down_pressed: bool,
    /// Is the Right Arrow pressed
    pub is_right_pressed: bool,
    /// Is the Left Arrow pressed
    pub is_left_pressed: bool,
}

impl KeyState {
    /// Initialize a new KeyState
    ///
    /// # Returns
    /// **KeyState**: KeyState with all attributes set to false
    fn new() -> Self {
        Self {
            is_start_pressed: false,
            is_select_pressed: false,
            is_a_pressed: false,
            is_b_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            is_right_pressed: false,
            is_left_pressed: false,
        }
    }

    /// Update the KeyState
    ///
    /// # Arguments
    /// **start (bool)**: Is the start button being pushed?
    /// **select (bool)**: Is the start button being pushed?
    /// **a (bool)**: Is the A button being pushed?
    /// **b (bool)**: Is the B button being pushed?
    /// **up (bool)**: Is the up button being pushed?
    /// **down (bool)**: Is the down button being pushed?
    /// **left (bool)**: Is the left button being pushed?
    /// **right (bool)**: Is the right button being pushed?
    fn update(
        &mut self,
        start: bool,
        select: bool,
        a: bool,
        b: bool,
        up: bool,
        down: bool,
        right: bool,
        left: bool
    ) {
        self.is_start_pressed = start;
        self.is_select_pressed = select;
        self.is_a_pressed = a;
        self.is_b_pressed = b;
        self.is_up_pressed = up;
        self.is_down_pressed = down;
        self.is_right_pressed = right;
        self.is_left_pressed = left;
        println!("{:?}", &self);
    }
}

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
                0;
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
        if !self.window.is_active() {
            println!("WINDOW IS NOT ACTIVE");
        }
        let keys = self.window.get_keys();
        println!("{:?}", keys);
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
