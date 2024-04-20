use futures::executor::block_on;

#[derive(Clone, Copy)]

pub struct BufferRaw<'a> {
    pub binding: u32,
    pub data: &'a [u8],
    pub is_read_only: bool,
    pub has_dynamic_offset: bool,
    // pub is_uniform: bool,
}

impl BufferRaw<'_> {
    pub fn new(binding: u32) -> Self {
        Self {
            binding,
            ..Default::default()
        }
    }

    pub fn from_data(binding: u32, data: &'static [u8]) -> Self {
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

    pub fn update(&mut self, buffer: &wgpu::Buffer, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device) {
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: self.data.len() as wgpu::BufferAddress,
            // Can be read to the CPU, and can be copied from the shader's storage buffer
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        encoder.copy_buffer_to_buffer(buffer, 0, &readback_buffer, 0, self.data.len() as u64);

        let slice = readback_buffer.slice(..);

        // to-do: replace sender, reciever with "one-cell"
        let (sender, reciever) = futures::channel::oneshot::channel();
        slice.map_async(wgpu::MapMode::Read, |res| {
            sender.send(res);
        });

        device.poll(wgpu::MaintainBase::Wait);
        
        let a = block_on(reciever).expect("comm failed").expect("buffer read failed");
        let buffer_res: &[u8] = &slice.get_mapped_range();
        println!("{:#?}", buffer_res); 
    }

}

impl Default for BufferRaw<'_> {
    fn default() -> Self {
        Self {
            binding: 0,
            data: &[],
            is_read_only: false,
            has_dynamic_offset: false,
        }
    }
}