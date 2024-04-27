use futures::executor::block_on;



#[derive(Clone)]

pub struct BufferRaw {
    pub binding: u32,
    pub data: Vec<u8>,
    pub is_read_only: bool,
    pub has_dynamic_offset: bool,
    // pub is_uniform: bool,
}

impl BufferRaw {
    pub fn new(binding: u32) -> Self {
        Self {
            binding,
            ..Default::default()
        }
    }

    pub fn from_data(binding: u32, data: Vec<u8>) -> Self {
        Self {
            binding,
            data,
            ..Default::default()
        }
    }

    pub fn set_read_only(&mut self, set: bool) {
        self.is_read_only = set;
    } 

    pub fn set_dynamic_offset(&mut self, set: bool) {
        self.has_dynamic_offset = set;
    }

    pub fn update(&mut self, buffer: &wgpu::Buffer, readback_buffer: &wgpu::Buffer, device: &wgpu::Device) {

        let slice = readback_buffer.slice(..);

        // to-do: replace sender, reciever with "one-cell"
        let (sender, reciever) = futures::channel::oneshot::channel();
        slice.map_async(wgpu::MapMode::Read, |res| {
            sender.send(res);
        });

        device.poll(wgpu::MaintainBase::Wait);
        
        let a = block_on(reciever).expect("comm failed").expect("buffer read failed");
        let data: Vec<u8> = slice.get_mapped_range().to_vec();
        // self.data = data;

    }

}

impl Default for BufferRaw {
    fn default() -> Self {
        Self {
            binding: 0,
            data: vec![],
            is_read_only: false,
            has_dynamic_offset: false,
        }
    }
}