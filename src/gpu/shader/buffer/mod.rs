use std::{marker::PhantomData, mem::transmute, ptr};
mod buffer_raw;
pub use buffer_raw::BufferRaw;
pub struct Buffer<T> {
    pub(crate) buffer_raw: BufferRaw,
    phantom_type: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new(binding: u32) -> Self {
        Self {
            buffer_raw: BufferRaw::new(binding),
            phantom_type: PhantomData
        }
    }

    pub fn data_raw(&self) -> &Vec<u8> {
        &self.buffer_raw.data
    }
    pub fn from(binding: u32, data: &T) -> Self {
        Self {
            buffer_raw: BufferRaw::from(binding, data),
            phantom_type: PhantomData
        }
    }

    pub fn data(&self) -> &[T] {
        u8_as_slice_of(&self.buffer_raw.data)
    }

}

pub fn any_as_u8<T>(data: &T) -> &[u8]{
    unsafe {
        std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of_val(data))
    }
}

pub fn u8_as_slice_of<T>(data: &[u8]) -> &[T] {
    unsafe {
        data.align_to::<T>().1
    }
}