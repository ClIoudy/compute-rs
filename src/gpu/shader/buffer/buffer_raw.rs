
pub struct BufferRaw {
    pub binding: u32,
    pub data: Vec<u8>,
    pub wgpu_buffer: Option<wgpu::Buffer>,
    pub is_borrowed: bool,
}

impl BufferRaw {
    pub(crate) fn new(binding: u32, data: &Vec<u8>, wgpu_buffer: wgpu::Buffer) -> Self {
        Self {
            binding,
            data: data.to_vec(),
            wgpu_buffer: Some(wgpu_buffer),
            is_borrowed: false,
        }
    }

    pub(crate) fn from<T>(binding: u32, data: &T) -> Self {
        Self {
            binding,
            data: any_as_u8(data).to_vec(),
            wgpu_buffer: None,
            is_borrowed: false,
        }
    }

    pub(crate) fn data<T>(&self) -> &[T] {
        u8_as_slice_of(&self.data)
    }

}

pub fn any_as_u8<T>(data: &T) -> &'static [u8]{
    unsafe {
        std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of_val(data))
    }
}

pub fn u8_as_slice_of<T>(data: &[u8]) -> &[T] {
    unsafe {
        data.align_to::<T>().1
    }
}