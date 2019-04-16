use super::gpu::*;
use gl::types::*;

pub struct Model {
    pub buffer: Option<VertexBufferObject>,
    pub array: Option<VertexArrayObject>,
    pub indices: Option<ElementBufferObject>,
}

impl Model {
    pub fn new_unloaded() -> Self {
        Self { 
            buffer: None, 
            indices: None,
            array : None, 
        }
    }

    pub fn from_data_and_vao<T>(data: &[T], indices: &[GLuint], vao: VertexArrayObject) -> Self {
        let vbo = VertexBufferObject::from_data(data, data.len());
        let ebo = ElementBufferObject::from_indices(indices, indices.len()); 
        Self {
            buffer: Some(vbo),
            array: Some(vao),
            indices: Some(ebo),
        }
    }

    pub fn from_data_and_layout<T>(data: &[T], indices: &[GLuint], attribs: &[Attribute]) -> Self {
        let vao = VertexArrayObject::from_layout(attribs);
        Model::from_data_and_vao(data, indices, vao)
    }

    pub fn bind(&self) -> Result<(), &'static str> {
        match (&self.buffer, &self.array, &self.indices) {
            (Some(vbo), Some(vao), Some(ebo)) => {
                unsafe {
                    ebo.bind();
                    vao.rebind_to_new_buffer(*vbo);
                    vbo.bind();
                }
                Ok(())
            }
            _ => Err("Model not loaded") 
        }
    }

    pub fn is_loaded(&self) -> bool {
        match (&self.buffer, &self.array, &self.indices) {
            (Some(_), Some(_), Some(_)) => true,
            _ => false,
        }
    }
}

