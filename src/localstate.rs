use std::collections::HashMap;

pub struct LocalState {
    pub clear_color: [f32; 4],
    pub models: HashMap<String, (bool, f32)>,
}

impl LocalState {
    pub fn new() -> Self {
        LocalState {
            clear_color: [0.5, 0.0, 0.3, 1.0],
            models: HashMap::new(),
        }
    }

    pub fn add_model(&mut self,_: &'static str) -> () {
       self.models.insert(
           "triangle".to_string(), 
           (true, 0.0)
       );
    }
}
