use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::ffi::CString;
use gl::types::*;

pub struct ShaderProg {
    pub id: u32
}

pub enum ShaderType {
    Vertex,
    Fragment
}

pub struct Shader {
    id: u32,
}

enum ShaderCompilationStatus {
    Success,
    Failure(String),
}

impl ShaderProg {
    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Self, String> {
        let mut prog_id = 0;
        unsafe {
            prog_id = gl::CreateProgram();
        }
        for shader in shaders.iter() {
           unsafe { 
               gl::AttachShader(prog_id, shader.id); 
               gl::DeleteShader(shader.id);
           } 
        }
        unsafe {
            gl::LinkProgram(prog_id);
        }

        match get_link_status(prog_id) {
            ShaderCompilationStatus::Success => {
                Ok(
                    Self { id: prog_id, }
                )
            },
            ShaderCompilationStatus::Failure(info_log) => {

                Err(
                    info_log
                )
            }
        }
    }

    pub unsafe fn activate(&self) -> () {
       gl::UseProgram(self.id); 
    }
}

impl Drop for ShaderProg {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Shader {
    pub fn from_source(file_path: &str, shader_type: ShaderType) -> Result<Self, String> {
        let mut shader_id;
        unsafe {
            shader_id = match shader_type {
                ShaderType::Vertex => gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::Fragment => gl::CreateShader(gl::FRAGMENT_SHADER),
            };
        }

        let file = File::open(file_path).map_err(|_| "Could not open shader source file!".to_string())?;
        let buf_reader = BufReader::new(file);
        let bytes = Box::new(buf_reader.bytes()
            .map(|res| res.expect("Error unwrapping byte in source file!"))
            .collect::<Vec<u8>>()
            );

        let file_contents_c_str = CString::new(*bytes)
            .map_err(|_| "Failed to cast bytes to C string")?;

        unsafe {
            gl::ShaderSource(shader_id, 1, &file_contents_c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }
        let status = get_compilation_status(shader_id);
        match status {
            ShaderCompilationStatus::Success => {
                Ok(
                    Self { id: shader_id }
                )
            }
            ShaderCompilationStatus::Failure(fail_log) => {
               Err(
                   fail_log
               ) 
            }
        }
    }
}

fn get_link_status(prog_id : u32) -> ShaderCompilationStatus {
    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut status);
    }
    if status == gl::TRUE as GLint {
        return ShaderCompilationStatus::Success;
    }

    let len: usize = 0;
    unsafe {
        gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut (len as i32));
    }
    let buf = vec![0 as u8;len]; 
    unsafe {
        gl::GetProgramInfoLog(
            prog_id, 
            len as i32, 
            std::ptr::null_mut(), 
            std::mem::transmute(&buf[0])
        ); 
    }

    ShaderCompilationStatus::Failure(
        std::str::from_utf8(&buf)
            .expect("Invalid utf8 encoding for shader log")
            .to_string()
    ) 
}

fn get_compilation_status(shader_id : u32) -> ShaderCompilationStatus {

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
    }

    if status == (gl::TRUE as GLint) {
        return ShaderCompilationStatus::Success;
    }
    let len: usize = 0;
    unsafe {
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, std::mem::transmute(&len));
    }

    let buf = vec![0 as u8;len]; 
    unsafe {
        gl::GetShaderInfoLog(
            shader_id, 
            len as i32, 
            std::ptr::null_mut(), 
            std::mem::transmute(&buf[0])
        ); 
    }

    ShaderCompilationStatus::Failure(
        std::str::from_utf8(&buf)
            .expect("Invalid utf8 encoding for shader log")
            .to_string()
    ) 
}


