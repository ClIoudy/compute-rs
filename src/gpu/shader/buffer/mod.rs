use std::marker::PhantomData;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

/// Buffer Struct for passing 
pub struct Buffer<T: Clone> {
    ty: PhantomData<T>,
    data: &'static [u8],
    binding: u32,
    label: Option<&'static str>
}

impl<T: Clone> Buffer<T> {
    pub fn new(data: &T, binding: u32) -> Self {
        Self {
            ty: PhantomData,
            data: any_as_u8(data),
            binding,
            label: None,
        }
    }

    pub(crate) fn data(&self) -> T {
        u8_as_any(self.data)
    }

    pub(crate) fn raw_data(&self) -> &[u8] {
        self.data
    }

    pub fn binding(&self) -> u32 {
        self.binding
    }

    pub fn label(&mut self, label: &'static str) {
        self.label = Some(label);
    }

    pub(crate) fn consume(&self) -> (BufferData, BufferId<T>) {
        let d = BufferData::new(self.data.to_vec());

        let id = BufferId::new(self.binding);

        (d, id)

    }

}

pub(crate) struct BufferData {
    pub(crate) data: Vec<u8>,
    pub(crate) label: Option<&'static str>,
}

impl BufferData {

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            label: None,
        }
    }

    #[inline]
    pub fn staging(&self, gpu: &crate::gpu::Gpu) -> wgpu::Buffer {
        let b = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: self.label,
            contents: &self.data,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });

        b
    }

    // pub fn size(&self) -> usize {
    //     self.data.len()
    // }
}

pub struct BufferId<T: Clone> {
    ty: PhantomData<T>,
    pub(crate) binding: u32,
}

impl<T: Clone> BufferId<T> {
    fn new(binding: u32) -> Self {
        Self {
            ty: PhantomData,
            binding,            
        }
    }
}

fn any_as_u8<T>(data: &T) -> &'static [u8]{
    unsafe {
        std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of_val(data))
    }
}

pub(crate) fn u8_as_any<T: Clone>(data: &[u8]) -> T {
    unsafe {
        data.align_to::<T>().1[0].clone()
    }
}
