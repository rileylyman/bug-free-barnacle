use gl::types::*;
use std::sync::Arc;

pub struct VertexArrayObject {
    pub id: u32,
    pub layout: Vec<Attribute>,
}

#[derive(Clone, Copy)]
pub struct Attribute {
    pub width: u8,
    pub stride: usize,
    pub start_idx: usize,
    pub ty: GLenum
}

pub struct ElementBufferObject {
    pub id: u32,
    pub num_elems: usize,
}

pub struct VertexBufferObject {
    pub id: u32
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }
        Self {
            id: vao_id,
            layout: Vec::new(),
        }
    }

    pub fn from_layout(attribs: &[Attribute]) -> Self {
        let mut result = VertexArrayObject::new();
        for (i, attr) in attribs.iter().enumerate() {
            unsafe {
                result.push_attrib(i, *attr); 
            }
        }
        result
    }

    pub unsafe fn push_attrib(&mut self, idx: usize, attr: Attribute) -> () {
        self.layout.push(attr);
        gl::BindVertexArray(self.id);
        VertexArrayObject::vertex_attrib_pointer(attr, idx);
        gl::EnableVertexAttribArray(idx as u32);
    }

    unsafe fn vertex_attrib_pointer(attr: Attribute, idx: usize) {
        gl::VertexAttribPointer(
            idx as u32,
            attr.width as i32,
            attr.ty,
            gl::FALSE,
            attr.stride as i32,
            std::mem::transmute(attr.start_idx),
        );
    }

    pub fn rebind_to_new_buffer(&self, vbo: Arc<VertexBufferObject>) -> () {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl::BindVertexArray(self.id);
        }
        for (i, attr) in self.layout.iter().enumerate() {
            unsafe {
                VertexArrayObject::vertex_attrib_pointer(*attr, i); 
                gl::EnableVertexAttribArray(i as u32);
            }
        }
    }

    pub unsafe fn bind(&self) -> () {
        gl::BindVertexArray(self.id);
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}

impl VertexBufferObject {
    pub fn new() -> Self {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
        }
        Self {
            id: vbo_id,
        }
    }

    pub fn from_data<T>(data: &[T], len: usize) -> Self {
        let result = VertexBufferObject::new();
        unsafe {
            result.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (len * std::mem::size_of::<T>()) as GLsizeiptr,
                std::mem::transmute(&data[0]),
                gl::STATIC_DRAW
            );
        }
        result
    }

    pub unsafe fn bind(&self) -> () {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

impl ElementBufferObject {
    pub fn new() -> Self {
        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
        }
        Self {
            id: ebo_id,
            num_elems: 0,
        }
    }

    pub fn from_indices(indices: &[GLuint], len: usize) -> Self {
        let mut result = ElementBufferObject::new();
        unsafe {
            result.bind();
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (len * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                std::mem::transmute(&indices[0]),
                gl::STATIC_DRAW
            );
        }
        result.num_elems = len;
        result
    }

    pub unsafe fn bind(&self) -> () {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
    }
}

impl Drop for ElementBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
