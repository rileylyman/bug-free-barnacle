#![allow(dead_code)]

mod gpu;
mod shader;
pub mod model;

use model::Model;
use super::math::Mat4;
use gpu::{Attribute, ElementBufferObject, VertexBufferObject, VertexArrayObject};
use shader::{Shader, ShaderProg, ShaderType::*};
use std::sync::Arc;

pub struct Renderer {
    wireframe     : bool,
    shaders       : Vec<Arc<ShaderProg>>,
    shader_idx    : i32,
    matrix        : Mat4, 
}

impl Renderer {
    pub fn init_only_once(window: &mut glfw::Window) -> Result<Self, &'static str> {
       gl::load_with(|s| window.get_proc_address(s) as *const _ ); 
       let (width, height) = window.get_size();
       unsafe {
           gl::Viewport(0, 0, width, height);
           gl::DebugMessageCallback(gl_debug_callback, std::ptr::null());
       }
       Ok(
           Renderer {
               wireframe     : false,
               shaders       : Vec::new(),
               shader_idx    : -1,
               matrix        : Mat4::identity(),
           }
       )
    }

    pub fn draw_model(&mut self, bound_model: &mut Model) -> Result<(), &'static str> {

        if !bound_model.is_loaded() {
            return Err("Model is not loaded");
        }
        if let Some(shader) = self.shaders.get(self.shader_idx as usize) {
            unsafe {
                //self.matrix = self.matrix.clone()
                //    .rotate_radians(0.0001, super::math::Axis::Z);
                //self.matrix.stretch(1.00001, 1.00001, 1.0);
                //self.matrix.translate(1.000001, 0.0, 0.0);
                shader.uniform_matrix4f("model", self.matrix.get()).unwrap();
                shader.uniform_float_array("c", &[0.4]).unwrap();
                let num_indices = if let Some(ref indices) = bound_model.indices {
                    indices.num_elems
                } else {
                    return Err("Something is wrong with model.is_loaded");
                };
                gl::DrawElements(
                    gl::TRIANGLES, 
                    //model.is_loaded guarantees this will not panic
                    num_indices as i32,
                    gl::UNSIGNED_INT, 
                    std::ptr::null()
                );
            }
        }
        Ok(())
    }

    pub fn use_shader_idx(&mut self, shader_idx: i32) -> Result<(), &'static str> {
        if let Some(shader) = self.shaders.get(shader_idx as usize) {
            self.shader_idx = shader_idx;
            unsafe {
                shader.activate();
            }
            Ok(())
        } else {
            Err("Could not activate given shader")
        }
    }

    pub fn toggle_wireframe(&mut self) -> () {
        if self.wireframe {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        } else {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        }
        self.wireframe = !self.wireframe;
    }

    pub unsafe fn clear(&self, color: [f32; 4]) -> () {
        gl::ClearColor(color[0], color[1], color[2], color[3]);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

}

pub fn load_models_from_local_state(r: &mut Renderer, local: &mut super::localstate::LocalState) -> Result<(), String> {
    let model = Model::from_data_and_layout(
        &vec![
            0.5 as gl::types::GLfloat, 0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 
             -0.5,  0.5, 0.0, 0.3, 0.4, 0.5,
        ],
        &vec![
            0, 1, 3,
            1, 2, 3
        ],
        &vec![
            Attribute { // attribute 0: pos
                width: 3,
                stride: 6 * std::mem::size_of::<gl::types::GLfloat>(),
                start_idx: 0,
                ty: gl::FLOAT,
            },
            Attribute { // attribute 1: color
                width: 3,
                stride: 6 * std::mem::size_of::<gl::types::GLfloat>(),
                start_idx: 3 * std::mem::size_of::<gl::types::GLfloat>(),
                ty: gl::FLOAT,
            }
        ]
    );

    local.add_model_moves(model);

    let vert_shader = Shader::from_source("./renderer/shaders/vert.glsl", Vertex)?;
    let frag_shader = Shader::from_source("./renderer/shaders/frag.glsl", Fragment)?;

    let shader = ShaderProg::from_shaders(vec![vert_shader, frag_shader])?;

    r.shaders.push(Arc::new(shader));
    r.use_shader_idx(0)?;
    Ok(())
}

pub fn draw_models(r: &mut Renderer, local: &mut super::localstate::LocalState) -> Result<(), &'static str> {
    for model in local.models.iter_mut() {
        model.bind()?;
        r.draw_model(model)?;
    }
    Ok(())
}

pub fn clear_screen(r: &Renderer, local: &super::localstate::LocalState) -> () {
    let clear_color = &local.clear_color;
    unsafe {
        r.clear((*clear_color).clone());
    }
}

extern "system" fn gl_debug_callback(
    _source: u32, 
    _type: u32, 
    _id: u32, 
    _sev: u32, 
    _length: i32, 
    _msg: *const i8, 
    _data: *mut std::ffi::c_void) 
-> () {
    error!("Reached error");
}
