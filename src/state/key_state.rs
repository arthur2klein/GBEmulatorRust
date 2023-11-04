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
    pub fn update(
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
