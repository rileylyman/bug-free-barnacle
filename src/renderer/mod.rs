mod gpu;
mod shader;

use gpu::{Attribute, ElementBufferObject, VertexBufferObject, VertexArrayObject};
use shader::{Shader, ShaderProg, ShaderType::*};

pub struct Renderer {
    vbos: Vec<VertexBufferObject>,
    vaos: Vec<VertexArrayObject>,
    ebos: Vec<ElementBufferObject>,
    shaders: Vec<ShaderProg>,
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
               vbos: Vec::new(),
               vaos: Vec::new(),
               ebos: Vec::new(),
               shaders: Vec::new(),
           }
       )
    }

    pub fn draw(&self, vao_idx: usize, vbo_idx: usize, ebo_idx: usize, shader_idx: usize) -> () {
        match (
            self.vaos.get(vao_idx),
            self.vbos.get(vbo_idx),
            self.ebos.get(ebo_idx),
            self.shaders.get(shader_idx))
        {
            (Some(vao), Some(vbo), Some(ebo), Some(shader)) => {
                unsafe {
                    vbo.bind();
                    vao.bind();
                    ebo.bind();
                    shader.activate();
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                }

            }
            _ => {
                error!("Could not find one of vao, vbo, or shader, from indices given!");
                return;
            }
        }
    }

    pub unsafe fn clear(&self, color: [f32; 4]) -> () {
        gl::ClearColor(color[0], color[1], color[2], color[3]);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn load_models_from_local_state(r: &mut Renderer, _local: &super::localstate::LocalState) -> () {
    let vbo = VertexBufferObject::from_data(
        &vec![
            0.5 as gl::types::GLfloat, 0.5, 0.0,
             0.5, -0.5, 0.0,
             -0.5, -0.5, 0.0, 
             -0.5,  0.5, 0.0,
        ],
        12
    );
    let ebo = ElementBufferObject::from_indices(
        &vec![
            0, 1, 3,
            1, 2, 3
        ],
        6
    );
    let vao = VertexArrayObject::from_layout(
        vec![
            Attribute {
                width: 3,
                stride: 3 * std::mem::size_of::<gl::types::GLfloat>(),
                start_idx: 0,
                ty: gl::FLOAT,
            },
        ]
    );

    let vert_shader = Shader::from_source("./renderer/shaders/vert.glsl", Vertex).expect("Vertex shader failed");
    let frag_shader = Shader::from_source("./renderer/shaders/frag.glsl", Fragment).expect("Fragment shader failed");

    let shader = ShaderProg::from_shaders(vec![vert_shader, frag_shader]).expect("Could not create shader");

    r.vaos.push(vao);
    r.vbos.push(vbo);
    r.ebos.push(ebo);
    r.shaders.push(shader);
}

pub fn draw_models(r: &mut Renderer, _local: &super::localstate::LocalState) -> () {
    r.draw(0, 0, 0, 0);
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
    println!("Reached error");
    error!("jey");
}
