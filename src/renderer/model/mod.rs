use super::gpu::*;
use gl::types::*;
use std::sync::Arc;

pub mod obj;

pub struct Model {
    pub buffer: Option<Arc<VertexBufferObject>>,
    pub array: Option<Arc<VertexArrayObject>>,
    pub indices: Option<Arc<ElementBufferObject>>,
    is_loaded: bool,
}

impl Model {
    pub fn new_unloaded() -> Self {
        Self { 
            buffer    : None, 
            indices   : None,
            array     : None, 
            is_loaded : false,
        }
    }

    pub fn from_data_and_vao<T>(data: &[T], indices: &[GLuint], vao: VertexArrayObject) -> Self {
        let vbo = VertexBufferObject::from_data(data, data.len());
        let ebo = ElementBufferObject::from_indices(indices, indices.len()); 
        Self {
            buffer  : Some(Arc::new(vbo)),
            array   : Some(Arc::new(vao)),
            indices : Some(Arc::new(ebo)),
            is_loaded: true,
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
                    vao.rebind_to_new_buffer(vbo.clone());
                    vbo.bind();
                }
                Ok(())
            }
            _ => Err("Model not loaded") 
        }
    }

    pub fn is_loaded(&mut self) -> bool {
        if !self.is_loaded {
            self.is_loaded = match (&self.buffer, &self.array, &self.indices) {
                (Some(_), Some(_), Some(_)) => true,
                _ => false,
            };
        }
        self.is_loaded
    }
}

