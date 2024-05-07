#![allow(warnings)]
pub mod buffer;
pub use buffer::*;
use wgpu::{util::DeviceExt, BufferUsages};

pub struct Shader<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    module: wgpu::ShaderModule,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    bind_group_entries: Vec<wgpu::BindGroupEntry<'a>>,
    bind_group: Option<wgpu::BindGroup>,
    pipline_layout: Option<wgpu::PipelineLayout>,
    buffers: Vec<(&'a mut Buffer, wgpu::Buffer)>,
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
        
        let mut staging_buffers = vec![];

        for (buffer, wgpu_buffer) in &self.buffers {
            let staging_buffer = self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: &buffer.data,
                    usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
                }
            );
            staging_buffers.push(staging_buffer)
        }

        let mut bind_group_layout_entries = vec![];
        let mut bind_group_entries = vec![];
        
        for i in 0..self.buffers.len() {
            let buffer = &self.buffers[i].0;
            let staging_buffer = &staging_buffers[i];

            let bind_group_entry = wgpu::BindGroupEntry {
                binding: buffer.binding,
                resource: staging_buffer.as_entire_binding()
            };

            let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
                binding: buffer.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                    has_dynamic_offset: false, 
                    min_binding_size: std::num::NonZeroU64::new(buffer.data.len() as u64) 
                },
                count: None,
            };

            bind_group_entries.push(bind_group_entry);
            bind_group_layout_entries.push(bind_group_layout_entry);
        }

        let bind_group_layout = self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { label: None, entries: bind_group_layout_entries.as_slice() }
        );
        
        let pipline_layout = self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        
        
        
        let bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor { 
                label: None, 
                layout: &bind_group_layout, 
                entries: bind_group_entries.as_slice() }
        );
        

        
        let compute_pipeline = self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor { 
                label: Some(entry_point),
                layout: Some(&pipline_layout),
                module: &self.module,  
                entry_point 
            }
        );

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(x, y, z)
        }

        for i in 0..self.buffers.len() {
            encoder.copy_buffer_to_buffer(
            &staging_buffers[i], 0,
            &self.buffers[i].1, 0,
            staging_buffers[i].size(),
            );
        }

        // encoder.copy_buffer_to_buffer(
        //     &staging_buffer, 0,
        //     &readback_buffer, 0,
        //     staging_buffer.size(),
        // );

        self.queue.submit(Some(encoder.finish()));
        // self.device.poll(wgpu::Maintain::Wait);
        
        for (buffer, wgpu_buffer) in self.buffers.iter_mut() {

            let buffer_slice = wgpu_buffer.slice(..);
            buffer_slice.map_async(wgpu::MapMode::Read, |x| () );
            self.device.poll(wgpu::Maintain::Wait);

            let data = buffer_slice.get_mapped_range().to_vec();

            buffer.data = data.clone();
            wgpu_buffer.unmap();
        }

    }


    pub fn add_buffer(&mut self, buffer: &'a mut Buffer) {      
        let wgpu_buffer = self.device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size: buffer.data.len() as wgpu::BufferAddress,
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ, 
                mapped_at_creation: false,
            }
        );

        let staging_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &buffer.data,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            }
        );

        



        self.buffers.push((buffer, wgpu_buffer));
    }
}