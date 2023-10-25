extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: u8 = 160; // Game Boy screen width
const HEIGHT: u8 = 144; // Game Boy screen height
const PIXEL_SIZE: usize = 5;

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
    }
}

pub struct Screen {
    /// Buffer for the screen
    buffer: Vec<u32>,
    /// Window to draw on
    window: Window,
    /// State of the key presses
    pub key_state: KeyState,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: vec![0xFFFFFF; PIXEL_SIZE * WIDTH as usize * HEIGHT as usize],
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
        }
    }

    pub fn update_key_press(&mut self) {
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
    }

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
