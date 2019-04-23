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

pub enum UniformType {
    Float(u8),
    UInt(u8),
    Int(u8),
}

enum ShaderCompilationStatus {
    Success,
    Failure(String),
}

impl ShaderProg {
    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Self, String> {
        let prog_id;
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
   
    #[allow(dead_code)]
    pub unsafe fn uniform_int_array(&self, name: &str, data: &[i32]) -> Result<(), String> {
        let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
        if location == -1 {
            return Err(format!("Could not find <{}> on shader program {}", name, self.id));
        }
        if data.len() == 1 {
            gl::Uniform1i(location, data[0]);
        } else if data.len() == 2 {
            gl::Uniform2i(location, data[0], data[1]);
        } else if data.len() == 3 {
            gl::Uniform3i(location, data[0], data[1], data[2]);
        } else if data.len() == 4 {
            gl::Uniform4i(location, data[0], data[1], data[2], data[3]);
        } else {
            return Err("Data is too long or too short.".to_string());
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub unsafe fn uniform_float_array(&self, name: &str, data: &[f32]) -> Result<(), String> {
        let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
        if location == -1 {
            return Err(format!("Could not find <{}> on shader program {}", name, self.id));
        }
        if data.len() == 1 {
            gl::Uniform1f(location, data[0]);
        } else if data.len() == 2 {
            gl::Uniform2f(location, data[0], data[1]);
        } else if data.len() == 3 {
            gl::Uniform3f(location, data[0], data[1], data[2]);
        } else if data.len() == 4 {
            gl::Uniform4f(location, data[0], data[1], data[2], data[3]);
        } else {
            return Err("Data is too long or too short.".to_string());
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub unsafe fn uniform_matrix4f(&self, name: &str, data: &[f32]) -> Result<(), String> {
        let location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
        if location == -1 {
            return Err(format!("Could not find <{}> on shader program {}", name, self.id));
        }
        if data.len() != 16 {
           return Err("Matrix not 4x4!".into()); 
        }
        println!("Data: {:?}", data);
        Ok(gl::UniformMatrix4fv(location, 1, gl::TRUE, std::mem::transmute(&data[0])))
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
        let shader_id;
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


