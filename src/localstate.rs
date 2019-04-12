
pub struct LocalState {
    pub clear_color: [f32; 4],
}

impl LocalState {
    pub fn new() -> Self {
        LocalState {
            clear_color: [0.5, 0.0, 0.3, 1.0],
        }
    }
}
