#[derive(Clone)]
/// Describe an object in the OAM
pub struct TileObject {
    /// y position of the object on the screen
    pub y_position: u8,
    /// x position of the object on the screen
    pub x_position: u8,
    /// Index of the tile in the VRAM
    pub tile_index: u8,
    /// Gives some information about the status of the object
    /// priority/y flip/x flip/dmg palette/unused/unused/unused/unused
    pub flags: u8
}

impl TileObject {
    /// Create a new object in the OAM with all its attributes at 0
    ///
    /// # Returns
    /// **TileObject**: New object with all attributes equal 0
    pub fn new() -> Self {
        Self {
            y_position: 0x00,
            x_position: 0x00,
            tile_index: 0x00,
            flags: 0x00,
        }
    }
    
    /// Does the object as priority for rendering?
    ///
    /// Reads the 1st bit of the flag to know if the object should be drawn on
    /// top or behind other objects
    ///
    /// # Returns
    /// **bool**: true iff the object has priority
    pub fn get_priority(&self) -> bool {
        self.flags & 0x80 == 0x80
    }

    /// Is the object tile applied with a vertical mirror?
    ///
    /// # Returns
    /// **bool**: true iff the object tile should be flipped vertically
    pub fn get_y_flip(&self) -> bool {
        self.flags & 0x40 == 0x40
    }

    /// Is the object tile applied with a horizontal mirror?
    ///
    /// # Returns
    /// **bool**: true iff the object tile should be flipped horizontally
    pub fn get_x_flip(&self) -> bool {
        self.flags & 0x20 == 0x20
    }

    /// Indicate what dmg palette to use
    ///
    /// # Returns
    /// **bool**: false for obj0 and true for obj1
    pub fn get_dmg_palette(&self) -> bool {
        self.flags & 0x10 == 0x10
    }
}
