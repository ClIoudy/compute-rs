mod buffer_raw;
use buffer_raw::BufferRaw;

pub struct Buffer<'a, T> {
    buffer: BufferRaw,
}

impl<T: Sized> Buffer<'_, T> {

    pub fn convert_back(&mut self) -> &T {
        let ptr = self.data_raw.as_ptr();
        let a = ptr as *mut T;
        unsafe {
            let ref res = *a;
            return res;
        }
    }

    pub fn from(binding: u32, data: T) -> Self {
        let buffer = BufferRaw::from_data(binding, any_as_u8_slice::<T>(&data));
         
        Self {
            buffer,
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