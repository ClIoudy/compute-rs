#![allow(warnings)]

mod buffer;
use buffer::{Buffer, BufferRaw};
use wgpu::util::DeviceExt;



pub struct Shader<'a> {
    device: &'a wgpu::Device,
    shader_module: wgpu::ShaderModule,
    // bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    // bind_group_entries: Vec<(u32, wgpu::Buffer)>,
    buffers: Vec<BufferRaw<'a>>,
}

impl<'a> Shader<'a> {
    pub fn new(device: &'a wgpu::Device, shader_module: wgpu::ShaderModule) -> Self {
        Self { device, shader_module, buffers: vec![] }
    }

    fn bind_layout_entry_of_buffer(buffer: &BufferRaw) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry { 
            binding: buffer.binding, 
            visibility: wgpu::ShaderStages::COMPUTE, 
            ty: (), 
            count: None
        }
    }

    fn bind_entry_of_buffer(buffer: &BufferRaw) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry { 
            binding: buffer.binding, 
            resource: () 
        }
    }

    pub fn dispatch(&mut self, entry_point: &str, x: u32, y: u32, z: u32) {
        
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = vec![];
        let bind_group_entries: Vec<wgpu::BindGroupEntry> = vec![];

        for buffer in &self.buffers {

        }

        let bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            // entires:  bind_group_layout_entries.as_slice()
            entries: &[
            ],
        });
        

        let bind_group = self.device.create_bind_group(
        &wgpu::BindGroupDescriptor { 
            label: None, 
            layout: &bind_group_layout, 
            // entires:  bind_group_entries.as_slice()
            entries: &[] 
        }
        );

        let compute_pipeline_layout = self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&bind_group_layout], push_constant_ranges: &[] }
        );

        let compute_pipeline = self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor { label: None, layout: Some(&compute_pipeline_layout), module: &self.shader_module, entry_point }
        );


        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None}
        );
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None }
        );

        let mut cpass = encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor { label: None, timestamp_writes: None }
        );



        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.set_pipeline(&compute_pipeline);
        cpass.dispatch_workgroups(x, y, z);


    }



    // pub fn add_buffer<T: Sized>(&mut self, buffer: &mut Buffer<T>) {
    //     // self.buffers.push(buffer);
    //     let buffer_binding_type;
    //     if buffer.is_uniform {  
    //         buffer_binding_type = wgpu::BufferBindingType::Uniform;
    //     } else {
    //         buffer_binding_type = wgpu::BufferBindingType::Storage { read_only: buffer.is_read_only };
    //     }

    //     let layout_entry = wgpu::BindGroupLayoutEntry {
    //         binding: buffer.binding,
    //         visibility: wgpu::ShaderStages::COMPUTE,
    //         ty: wgpu::BindingType::Buffer { 
    //             ty: wgpu::BufferBindingType::Storage { 
    //                 read_only: buffer.is_read_only 
    //             }, has_dynamic_offset: buffer.has_dynamic_offset, 
    //             min_binding_size: std::num::NonZeroU64::new(1) 
    //         },
    //         count: None
    //     };

    //     let wgpu_buffer = self.device.create_buffer_init(
    //         &wgpu::util::BufferInitDescriptor {
    //             label: None,
    //             contents: buffer.data_raw,
    //             usage: wgpu::BufferUsages::STORAGE 
    //                 | wgpu::BufferUsages::COPY_SRC 
    //         }
    //     );


    //     // self.bind_group_layout_entries.push(layout_entry);
    //     // self.bind_group_entries.push((buffer.binding, wgpu_buffer));

    // }

}