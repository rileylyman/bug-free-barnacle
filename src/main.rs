#![feature(rustc_private)]
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate glfw;
extern crate gl; 
extern crate packed_simd;

mod input;
mod window;
mod renderer;
mod localstate;
mod math;

use input::{get_inputs, UserInput::CloseRequested};
use window::WindowState;
use renderer::{clear_screen, Renderer, load_models_from_local_state, draw_models};
use localstate::LocalState;
use math::Mat4;

use glfw::Context;

fn main() -> Result<(), &'static str> {
    simple_logger::init().unwrap();

    let mut window_state = WindowState::default();
    let mut renderer = Renderer::init_only_once(&mut window_state.window).map_err(|_| "Could not initialize the renderer!")?; 
    let mut local_state = LocalState::new();

    //renderer.toggle_wireframe();
    load_models_from_local_state(&mut renderer, &mut local_state);
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
        clear_screen(&mut renderer, &local_state);
        draw_models(&mut renderer, &mut local_state);
        window_state.window.swap_buffers();
    }
    Ok(())
}
