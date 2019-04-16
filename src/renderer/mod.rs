mod gpu;
mod shader;
pub mod model;

use model::Model;
use gpu::{Attribute, ElementBufferObject, VertexBufferObject, VertexArrayObject};
use shader::{Shader, ShaderProg, ShaderType::*};

pub struct Renderer {
    wireframe: bool,
    shaders: Vec<ShaderProg>,
    current_shader: usize,
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
               wireframe: false,
               shaders: Vec::new(),
               current_shader: 0,
           }
       )
    }

    pub fn draw_model(&self, model: &Model) -> Result<(), &'static str> {

        if model.is_loaded() {
            if let Some(shader) = self.shaders.get(self.current_shader) {
                unsafe {
                    shader.activate();
                }
            } else {
                return Err("Invalid current shader index");
            }
            unsafe {
                model.bind()?;
                gl::DrawElements(
                    gl::TRIANGLES, 
                    //model.is_loaded guarantees this will not panic
                    model.indices
                        .expect("Theres a bug in Model::is_loaded")
                        .num_elems as i32, 
                    gl::UNSIGNED_INT, 
                    std::ptr::null()
                );
            }
        }
        Ok(())
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

    pub fn set_current_shader(&mut self, new_idx: usize) -> () {
        if let Some(shader) = self.shaders.get(new_idx) {
            self.current_shader = new_idx;
        }
    }
}

pub fn load_models_from_local_state(r: &mut Renderer, local: &mut super::localstate::LocalState) -> () {
    let model = Model::from_data_and_layout(
        &vec![
            0.5 as gl::types::GLfloat, 0.5, 0.0,
             0.5, -0.5, 0.0,
             -0.5, -0.5, 0.0, 
             -0.5,  0.5, 0.0,
        ],
        &vec![
            0, 1, 3,
            1, 2, 3
        ],
        &vec![
            Attribute {
                width: 3,
                stride: 3 * std::mem::size_of::<gl::types::GLfloat>(),
                start_idx: 0,
                ty: gl::FLOAT,
            },
        ]
    );

    local.add_model_moves(model);

    let vert_shader = Shader::from_source("./renderer/shaders/vert.glsl", Vertex).expect("Vertex shader failed");
    let frag_shader = Shader::from_source("./renderer/shaders/frag.glsl", Fragment).expect("Fragment shader failed");

    let shader = ShaderProg::from_shaders(vec![vert_shader, frag_shader]).expect("Could not create shader");

    r.shaders.push(shader);
    r.set_current_shader(0);
}

pub fn draw_models(r: &mut Renderer, local: &super::localstate::LocalState) -> () {
    for model in local.models.iter() {
        r.draw_model(model);
    }
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
