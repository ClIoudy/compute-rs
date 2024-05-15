use std::{fmt::Debug, marker::PhantomData, mem::transmute, ptr};
mod buffer_raw;
pub use buffer_raw::*;

pub struct Buffer<T> {
    pub(crate) buffer_raw: BufferRaw,
    phantom_type: PhantomData<T>,
}

impl<T> Buffer<T> {

    // retrieves the raw (byte) buffer data
    pub fn data_raw(&self) -> &Vec<u8> {
        &self.buffer_raw.data
    }

    // creates a buffer with a binding from data of any type
    pub fn from(binding: u32, data: &T) -> Self {
        Self {
            buffer_raw: BufferRaw::from(binding, data),
            phantom_type: PhantomData
        }
    }

    // creates a buffer with a binding from data of any type
    pub fn data(&self) -> &[T] {
        buffer_raw::u8_as_slice_of(&self.buffer_raw.data)
    }

    // changes the buffers binding
    pub fn set_binding(&mut self, binding: u32) {
        self.buffer_raw.binding = binding;
    }

    // overwrites the buffer data
    // this will not do anything if the buffer is already borrowed by a shader
    pub fn set_data(&mut self, data: &T) {
        if self.buffer_raw.is_borrowed {
            return;
        }
        self.buffer_raw.data = any_as_u8(data).to_vec();
    }

    // overwrites the buffer data
    // this will not do anything if the buffer is already borrowed by a shader
    pub fn set_data_raw(&mut self, data: Vec<u8>) {
        if self.buffer_raw.is_borrowed {
            return;
        }
        self.buffer_raw.data = data;
    }
    
}

impl<T: Debug> std::fmt::Display for Buffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data()[0])
    }
}