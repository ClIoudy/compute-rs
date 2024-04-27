use std::marker::PhantomData;

mod buffer_raw;
pub use buffer_raw::BufferRaw;

pub struct Buffer<T: Sized> {
    pub raw: BufferRaw,
    stored_type: PhantomData<T>
}

impl<T: Sized> Buffer<T> {

    // pub fn data(&mut self) -> &T {
    //     let ptr = self.raw.data.as_slice().as_ptr();
    //     let a = ptr as *mut T;
    //     unsafe {
    //         let ref res = *a;
    //         return res;
    //     }
    // }

    // pub fn data_u32(&self) -> Vec<u32> {
    //     return Vec::from_raw_parts(ptr, length, capacity)
    // }

    pub fn data(&self) -> &T {
        let p: *const T = self.raw.data.as_slice().as_ptr() as *const T;
        unsafe { &*p }
    }
    // pub fn convert_back(&self) -> &T {
    //     let ptr = self.raw.data.as_ptr();
    //     let a = ptr as *mut T;
    //     unsafe {
    //         let ref res = *a;
    //         return res;
    //     }
    // }

    pub fn from(binding: u32, data: T) -> Self {
        let raw_data = any_as_u8_slice::<T>(data);
        let buffer = BufferRaw::from_data(binding, raw_data.to_vec());
        
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

pub fn convert_back_to<T: Sized>(data: &[u8]) -> &T {
    let ptr = data.as_ptr();
    let a = ptr as *mut T;
    unsafe {
        let ref res = *a;
        return res;
    }
}