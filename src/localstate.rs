use super::renderer::model::Model;

pub struct LocalState {
    pub clear_color: [f32; 4],
    pub models: Vec<Model>,
}

impl LocalState {
    pub fn new() -> Self {
        LocalState {
            clear_color: [0.5, 0.0, 0.3, 1.0],
            models: Vec::new(),
        }
    }

    pub fn add_model_moves(&mut self, model: Model) -> () {
        self.models.push(model);
    }
}
