#![feature(rustc_private)]
#[macro_use]
extern crate log;
extern crate glfw;
extern crate gl; 

mod input;
mod window;
mod renderer;
mod localstate;

use input::{get_inputs, UserInput::CloseRequested};
use window::WindowState;
use renderer::{clear_screen, Renderer};
use localstate::LocalState;

use glfw::Context;

fn main() -> Result<(), &'static str> {
    let mut window_state = WindowState::default();
    let mut renderer = Renderer::init_only_once(&mut window_state.window).map_err(|_| "Could not initialize the renderer!")?; 
    let local_state = LocalState::new();

    while !window_state.should_close() {
        let mut inputs = get_inputs(&mut window_state)?;
        for input in inputs.get_all().iter() {
            match input {
                CloseRequested => { 
                    window_state.close();
                }
                _ => {}
            }
        }

        window_state.window.swap_buffers();
        clear_screen(&mut renderer, &local_state);
    }
    Ok(())
}
