#![allow(warnings)]
mod shader;
pub use futures::executor::block_on;

use self::shader::Shader;

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Gpu {
    pub fn new() -> Self {
        block_on(Self::async_new())
    }

    pub async fn async_new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .expect("Failed to find an appropriate adapter");

        // Use instance to create device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor::default(),
                None,
            )
            .await
            .expect("Failed to create device");
        drop(instance);
        drop(adapter);

        Self {
            device,
            queue,
        }

    }

    pub fn new_shader(&self, shader_binary: &[u8]) -> Shader {

        let module = unsafe {
            self.device.create_shader_module(
                wgpu::ShaderModuleDescriptor { 
                    label: None, 
                    source: wgpu::ShaderSource::SpirV(wgpu::util::make_spirv_raw(&shader_binary)) 
                }
            )
        };

        Shader::new(&self.device, module)
    }

    // pub fn execute(&self, shader: &[u8], entry_point: &str) {
    //     let shader_binary = wgpu::ShaderModuleDescriptor {
    //         label: None,
    //         source: wgpu::ShaderSource::SpirV(
    //             wgpu::util::make_spirv_raw(shader)
    //         ),
    //     };

    //     let module = self.device.create_shader_module(shader_binary);

    //     // let pipeline_layout = self.device.create_pipeline_layout(
    //     //     &wgpu::PipelineLayoutDescriptor::default()
    //     // );

    //     let compute_pipepline = self.device.create_compute_pipeline(
    //         &wgpu::ComputePipelineDescriptor { label: None, layout: None, module: &module, entry_point }
    //         // &wgpu::ComputePipelineDescriptor { label: None, layout: Some(&pipeline_layout), module: &module, entry_point }
    //     );

    //     let bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[] });

    //     let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &bind_group_layout, entries: &[]});

    //     let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    //     let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
    //     cpass.set_pipeline(&compute_pipepline);
    //     cpass.set_bind_group(0, &bind_group, &[]);
    //     cpass.dispatch_workgroups(64, 1, 1);
        
    // }

    // pub fn wait_for() {

    // }

    // pub fn create_buffer_from(&self, contents: &[u8] ) {
    //     let result = self.device.create_buffer_init(
    //         &wgpu::util::BufferInitDescriptor {
    //             label: None,
    //             contents,
    //             usage: 
    //         }
    //     );
    // }
}