#![allow(warnings)]
pub mod buffer;
pub use buffer::*;
use wgpu::util::DeviceExt;

pub struct Shader<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    module: wgpu::ShaderModule,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    bind_group_entries: Vec<wgpu::BindGroupEntry<'a>>,
    bind_group: Option<wgpu::BindGroup>,
    pipline_layout: Option<wgpu::PipelineLayout>,
    buffers: Vec<&'a mut Buffer>,
}

impl<'a> Shader<'a> {
    pub fn new(device: &'a wgpu::Device, queue: &'a wgpu::Queue, module: wgpu::ShaderModule) -> Self {
        Self {
            device,
            queue,
            module,
            bind_group_layout: None,
            bind_group_layout_entries: vec![],
            bind_group_entries: vec![],
            bind_group: None,
            pipline_layout: None,
            buffers: vec![]
        }
    }

    

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32, entry_point: &str) {

        if self.pipline_layout.is_none() {

            if self.bind_group_layout.is_none() {
                self.bind_group_layout = Some(self.device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor { label: None, entries: self.bind_group_layout_entries.as_slice() }
                ));
            }

            self.pipline_layout = Some(self.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[self.bind_group_layout.as_ref().unwrap()],
                    push_constant_ranges: &[],
                }
            ));
        }
        
        if self.bind_group.is_none() {
            self.bind_group = Some(self.device.create_bind_group(
                &wgpu::BindGroupDescriptor { 
                    label: None, 
                    layout: self.bind_group_layout.as_ref().unwrap(), 
                    entries: self.bind_group_entries.as_slice() }
            ));
        }


        let compute_pipeline = self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor { 
                label: Some(entry_point),
                layout: Some(self.pipline_layout.as_ref().unwrap()),
                module: &self.module,  
                entry_point 
            }
        );

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
            cpass.dispatch_workgroups(x, y, z)
        }


        // encoder.copy_buffer_to_buffer(
        //     &staging_buffer, 0,
        //     &readback_buffer, 0,
        //     staging_buffer.size(),
        // );

        // self.queue.submit(Some(encoder.finish()));
        // self.device.poll(wgpu::Maintain::Wait);
        // let buffer_slice = readback_buffer.slice(..);
        // buffer_slice.map_async(wgpu::MapMode::Read, |x| () );
        // self.device.poll(wgpu::Maintain::Wait);

        // let data = buffer_slice.get_mapped_range().to_vec();


        // self.buffers[0].data = data.clone();
    }


    pub fn add_buffer(&mut self, buffer: &'a mut Buffer) {        
        self.buffers.push(buffer);
    }
}