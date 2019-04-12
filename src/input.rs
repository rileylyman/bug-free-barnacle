use std::vec::Vec;
use super::window::WindowState;

pub enum UserInput {
    CloseRequested,
}

pub struct Inputs {
    inputs: Vec<UserInput>,
}

impl Inputs {
    pub fn get_all(&mut self) -> Vec<UserInput> {
        let mut result = Vec::new();
        while self.inputs.len() > 0 {
            match self.inputs.pop() {
                Some(input) => result.push(input),
                _ => {}
            }
        }
        result
    }
}

pub fn get_inputs(state: &mut WindowState) -> Result<Inputs, &'static str> {
    use UserInput::CloseRequested; 
    let mut inputs = Inputs { inputs: Vec::new(), };
    for (_, event) in state.poll_events() {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                inputs.inputs.push(CloseRequested); 
                println!("Escape pressed");
            }
            _ => {},
        }
    }
    if state.window.should_close() {
        inputs.inputs.push(CloseRequested);
    }
    Ok(inputs)
}
