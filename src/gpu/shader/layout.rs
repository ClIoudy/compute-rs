use std::collections::HashMap;

use crate::gpu::Gpu;



impl super::Shader {
    
    pub fn reset_layout(&mut self) {
        self.staging_buffers = vec![];
        self.pipeline_layout = None;
        self.bind_group = None;
    }

    pub fn reset_buffers(&mut self) {
        self.buffers = HashMap::new();
    }

    /// tells the Shader that it is not ready for dispatch yet
    /// and a freeze_layout call is required before it can be dispatched again
    pub fn unready(&mut self) {
        self.is_ready = false;
    }

    pub fn freeze_layout(&mut self, gpu: &Gpu) {
        self.reset_layout();

        let staging_buffers = self.create_staging_buffers(gpu);
        let (bind_entries, bind_layout_entries) = Self::bind_entries(&staging_buffers);

        let bind_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some(&self.object_label()), 
            entries: &bind_layout_entries 
        });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&self.object_label()), 
            layout: &bind_layout, 
            entries: &bind_entries 
        });

        let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some(&self.object_label()), 
            bind_group_layouts: &[&bind_layout], 
            push_constant_ranges: &[] 
        });

        self.pipeline_layout = Some(pipeline_layout);
        
        self.bind_group = Some(bind_group);

        self.staging_buffers = staging_buffers;

        self.is_ready = true;

    }

    fn create_staging_buffers(&self, gpu: &Gpu) -> Vec<(wgpu::Buffer, u32)> {
        use wgpu::{BufferUsages, util::{BufferInitDescriptor, DeviceExt}};

        let mut staging_buffers = vec![];

        for (binding, buffer) in &self.buffers {
            let wgpu_buffer = gpu.device.create_buffer_init(
                &BufferInitDescriptor {
                    label: buffer.label,
                    contents: &buffer.data,
                    usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
                }
            );

            // let wgpu_buffer = buffer.staging(&gpu);
            
            staging_buffers.push((wgpu_buffer, *binding));
            
        }

        staging_buffers
    }

    fn bind_entries(staging_buffers: &Vec<(wgpu::Buffer, u32)>) -> (Vec<wgpu::BindGroupEntry>, Vec<wgpu::BindGroupLayoutEntry>) {
        
        let mut bind_group_layout_entries = vec![];
        let mut bind_group_entries = vec![];
        
        for (staging_buffer, binding) in staging_buffers {

            let bind_group_entry = wgpu::BindGroupEntry {
                binding: *binding,
                resource: staging_buffer.as_entire_binding(),
            };

            let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
                binding: *binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false, 
                    min_binding_size: std::num::NonZeroU64::new(staging_buffer.size()) 
                },
                count: None,
            };

            bind_group_entries.push(bind_group_entry);
            bind_group_layout_entries.push(bind_group_layout_entry);

        }

        (bind_group_entries, bind_group_layout_entries)
        
    }

}