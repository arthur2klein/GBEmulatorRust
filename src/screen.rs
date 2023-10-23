extern crate minifb;

use minifb::{Key, Window, WindowOptions};

struct Screen {
    // TODO
}

impl Screen {
    // TODO
}

pub fn display() {
    const WIDTH: usize = 160; // Game Boy screen width
    const HEIGHT: usize = 144; // Game Boy screen height
    const TILE_SIZE: usize = 8; // Tile size in pixels

    // Create a buffer for the screen
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Create a window
    let mut window = Window::new("Game Boy Graphics", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Tile data - Each tile is represented as a 2D array of color indices
    let tile_data: [[u32; TILE_SIZE]; TILE_SIZE] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the screen
        for pixel in buffer.iter_mut() {
            *pixel = 0; // Clear to black
        }

        // Draw tiles to the buffer
        for y in 0..HEIGHT / TILE_SIZE {
            for x in 0..WIDTH / TILE_SIZE {
                let tile_x = x * TILE_SIZE;
                let tile_y = y * TILE_SIZE;

                for row in 0..TILE_SIZE {
                    for col in 0..TILE_SIZE {
                        let color_index = tile_data[row][col];
                        let pixel_x = tile_x + col;
                        let pixel_y = tile_y + row;

                        // Draw the pixel with the specified color index
                        if color_index == 1 {
                            buffer[pixel_y * WIDTH + pixel_x] = 0xFFFFFF; // White color
                        }
                    }
                }
            }
        }
        // Update the window
        window
            .update_with_buffer(&buffer)
            .unwrap_or_else(|e| {
                panic!("{}", e);
        });
    }
}
