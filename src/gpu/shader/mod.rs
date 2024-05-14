#![allow(warnings)]
pub mod buffer;
pub use buffer::*;
use wgpu::{util::DeviceExt, BufferUsages};

pub struct Shader<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    module: wgpu::ShaderModule,
    staging_buffers: Vec<BufferRaw>,
    abc: Vec<&'a mut BufferRaw>,
}

impl<'a> Shader<'a> {
    pub fn new(device: &'a wgpu::Device, queue: &'a wgpu::Queue, module: wgpu::ShaderModule) -> Self {
        Self {
            device,
            queue,
            module,
            staging_buffers: vec![],
            abc: vec![],
        }
    }

    

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32, entry_point: &str) {        

        let mut bindgroup_layout_entries = vec![];
        let mut bindgroup_entries = vec![];

        for buffer in &self.staging_buffers {

            let wgpu_buffer = buffer.wgpu_buffer.as_ref().unwrap();

            let bindgroup_entry = wgpu::BindGroupEntry {
                binding: buffer.binding,
                resource: wgpu_buffer.as_entire_binding(),
            };

            let bindgroup_layout_entry = wgpu::BindGroupLayoutEntry {
                binding: buffer.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false, 
                    min_binding_size: std::num::NonZeroU64::new(wgpu_buffer.size()) 
                },
                count: None,
            };

            bindgroup_entries.push(bindgroup_entry);
            bindgroup_layout_entries.push(bindgroup_layout_entry);
        }

        let bindgroup_layout = self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { 
                label: None,
                entries: &bindgroup_layout_entries
            }
        );

        let bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor { 
                label: None, 
                layout: &bindgroup_layout,
                entries: bindgroup_entries.as_slice(),
            }
        );

        let pipline_layout = self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { 
                label: None, 
                bind_group_layouts: &[&bindgroup_layout], 
                push_constant_ranges: &[] }
        );

        let compute_pipeline = self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor { 
                label: None,
                layout: Some(&pipline_layout), 
                module: &self.module, 
                entry_point
            }
        );
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        
        {
            let mut cpass = encoder.begin_compute_pass(
                &wgpu::ComputePassDescriptor { 
                    label: None, 
                    timestamp_writes: None 
                }
            );
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(x, y, z)
        }

        let mut wgpu_buffers = vec![];

        for staging_buffer in &self.staging_buffers {

            let staging_buffer = staging_buffer.wgpu_buffer.as_ref().unwrap();

            let wgpu_buffer = self.device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: None,
                    size: staging_buffer.size(),
                    usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }
            );

            encoder.copy_buffer_to_buffer(
                &staging_buffer, 0,
                &wgpu_buffer, 0,
                staging_buffer.size(),
            );

            wgpu_buffers.push(wgpu_buffer);

        }


        self.queue.submit(Some(encoder.finish()));
        self.device.poll(wgpu::Maintain::Wait);
        

        for i in 0..wgpu_buffers.len() {
            let wgpu_buffer = &wgpu_buffers[i];

            let buffer_slice = wgpu_buffer.slice(..);
            buffer_slice.map_async(wgpu::MapMode::Read, |x| () );
            self.device.poll(wgpu::Maintain::Wait);

            let data = buffer_slice.get_mapped_range().to_vec();
            println!("{:?}", u8_as_slice_of::<u32>(&data));
            self.abc[i].data = data;

        }   
        
        self.abc = vec![];
        
    }

    pub fn add_buffer<T>(&mut self, buffer: &'a mut Buffer<T>) {
        self.add_buffer_unowned(buffer);
        self.abc.push(&mut buffer.buffer_raw);
    }

    pub fn add_buffer_unowned<T>(&mut self, buffer: &Buffer<T>) {
        let staging_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: buffer.data_raw(),
                usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            }
        );

        let staging_buffer = BufferRaw::new(buffer.buffer_raw.binding, buffer.data_raw(), staging_buffer);
        self.staging_buffers.push(staging_buffer);
    }
}