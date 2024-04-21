#![allow(warnings)]
pub mod shader;
pub use shader::buffers::{Buffer};
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

        Shader::new(&self.device, module, &self.queue)
    }

}