pub struct BufferRaw {
    
}

pub struct Buffer<'a> {
    pub binding: u32,
    pub is_read_only: bool,
    pub has_dynamic_offset: bool,
    pub is_uniform: bool,
    pub data_raw: &'a [u8],
}



// but they SHOULD have a type, so that they can be used as normal values basically, no need for intermediate values etc, just 
// creating a buffer -> sending it off -> getting it back -> using it directly as values

impl Buffer<'_> {

    pub fn convert_back<T: Sized>(&mut self) -> &T {
        let ptr = self.data_raw.as_ptr();
        let a = ptr as *mut T;
        unsafe {
            let ref res = *a;
            return res;
        }
    }

    

    // // try to find some way to use a generic type to allow for buffers to be created directly from data
    // pub fn from_slice(binding: u32, data: &[u8]) -> Self {
    //     // this should return a buffer holding all the neccesary information needed by shader to create a real buffer.
    //     Self {

    //     }
    // }

    // pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        
    //     core::slice::from_raw_parts(
    //         (p as *const T) as *const u8,
    //         ::core::mem::size_of::<T>(),
    //     )
    // }

    // pub unsafe fn from_any<T: Sized>(binding: u32, data: &T) -> Self {
    //     Self::from_slice(binding, Self::any_as_u8_slice(data))
    // }

    // // maybe use something like this to define which use cases should be present
    // // shader can then later use these for creating a real buffer
    // pub fn set_uses(&mut self) {
        
    // }
}

struct B<T: Sized> {
    data: T,
    binding: u32
}

impl<T> B<T> {

}
