use std::marker::PhantomData;

mod buffer_raw;
pub use buffer_raw::BufferRaw;

pub struct Buffer<'a, T: Sized> {
    buffer: BufferRaw<'a>,
    stored_type: PhantomData<T>
}

impl<'a, T: Sized> Buffer<'a, T> {

    pub fn convert_back(&mut self) -> &T {
        let ptr = self.buffer.data.as_ptr();
        let a = ptr as *mut T;
        unsafe {
            let ref res = *a;
            return res;
        }
    }

    pub fn from(binding: u32, data: &'static T) -> Self {
        let buffer = BufferRaw::from_data(binding, any_as_u8_slice::<T>(&data));
        
        Self {
            buffer,
            stored_type: PhantomData     
        }
    }
}


pub fn any_as_u8_slice<T: Sized>(any: &T) -> &[u8] {
    unsafe{ 
        core::slice::from_raw_parts(
            (any as *const T) as *const u8,
            ::core::mem::size_of::<T>(),
        )
    }
}