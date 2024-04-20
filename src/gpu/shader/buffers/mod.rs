use std::marker::PhantomData;

mod buffer_raw;
pub use buffer_raw::BufferRaw;

pub struct Buffer<'a, T: Sized> {
    pub raw: BufferRaw<'a>,
    stored_type: PhantomData<T>
}

impl<'a, T: Sized + 'static> Buffer<'a, T> {

    pub fn convert_back(&mut self) -> &T {
        let ptr = self.raw.data.as_ptr();
        let a = ptr as *mut T;
        unsafe {
            let ref res = *a;
            return res;
        }
    }

    pub fn from(binding: u32, data: T) -> Self {
        let raw_data = any_as_u8_slice::<T>(data);
        let buffer = BufferRaw::from_data(binding, raw_data);
        
        Self {
            raw: buffer,
            stored_type: PhantomData     
        }
    }
}


pub fn any_as_u8_slice<T: Sized>(any: T) -> &'static [u8] {
    unsafe {
        core::slice::from_raw_parts(
            (&any as *const T) as *const u8,
            ::core::mem::size_of::<T>(),
        )
    }
}