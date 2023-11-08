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
    pub fn new() -> Self {
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
}
