use gl::types::*;

pub struct VertexArrayObject {
    pub id: u32,
}

pub struct Attribute {
    pub width: u8,
    pub stride: usize,
    pub start_idx: usize,
    pub ty: GLenum
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }
        Self {
            id: vao_id,
        }
    }

    pub fn from_layout(attribs: Vec<Attribute>) -> Self {
        let result = VertexArrayObject::new();
        for (i, ref attr) in attribs.iter().enumerate() {
            unsafe {
                result.push_attrib(i, attr); 
            }
        }
        result
    }

    pub unsafe fn push_attrib(&self, idx: usize, attr: &Attribute) -> () {
       gl::BindVertexArray(self.id);
       gl::VertexAttribPointer(
           idx as u32,
           attr.width as i32,
           attr.ty,
           gl::FALSE,
           attr.stride as i32,
           std::mem::transmute(attr.start_idx),
       );
       gl::EnableVertexAttribArray(idx as u32);
    }

    pub unsafe fn bind(&self) -> () {
        gl::BindVertexArray(self.id);
    }
}

pub struct VertexBufferObject {
    pub id: u32
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

    pub fn from_data<T: std::fmt::Debug>(data: &[T], len: usize) -> Self {
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
