use crate::gpu::{shader::BufferData, Gpu};

use super::BoundBuffer;

impl super::Shader {
    pub fn dispatch(&mut self, entry_point: &str, x: u32, y: u32, z: u32, gpu: &Gpu) {
        
        if !self.is_ready {
            return;
        }
        
        // let now = std::time::Instant::now();
        let compute_pipeline = gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { 
            label: Some(&self.object_label()), 
            layout: self.pipeline_layout.as_ref(),
            module: &self.module, 
            entry_point
        });
        // println!("creating compute pipeline took:       {:?}", now.elapsed());

        
        // let now = std::time::Instant::now();
        let mut encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some(&self.object_label()) }
        );
        // println!("creating encoder took: {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        let mut cpass = encoder.begin_compute_pass(
                &wgpu::ComputePassDescriptor { 
                label: Some(&self.object_label()),
                timestamp_writes: None 
            }
        );
        // println!("creating cpass took:                  {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
        // println!("setting pipeline and bind group took: {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        cpass.dispatch_workgroups(x, y, z);
        drop(cpass);
        // println!("actual dispatch took:                 {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        let read_buffers = self.create_read_buffers(gpu, &mut encoder);
        // println!("creating read buffers took:           {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        gpu.queue.submit(Some(encoder.finish()));
        gpu.device.poll(wgpu::Maintain::Wait);
        // println!("polling took:                         {:?}", now.elapsed());

        // let now = std::time::Instant::now();
        self.read_back(gpu, read_buffers);
        // println!("read back took:                       {:?}", now.elapsed());

    }

    fn create_read_buffers(&mut self, gpu: &Gpu, encoder: &mut wgpu::CommandEncoder) -> Vec<BoundBuffer> {
        let mut read_buffers = vec![];
        
        for (staging, binding) in &self.staging_buffers {
            let read_back = gpu.device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: Some(&self.read_buffers_label(binding)),
                    size: staging.size(),
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }
            );
            
            encoder.copy_buffer_to_buffer(
                &staging, 0, 
                &read_back, 0, 
                staging.size()
            );
            

            read_buffers.push((read_back, *binding));
        }

        read_buffers
    }

    fn read_back(&mut self, gpu: &Gpu, read_back_buffers: Vec<BoundBuffer>) {
        for (buffer, binding) in read_back_buffers {
            let buffer_slice = buffer.slice(..);
            buffer_slice.map_async(wgpu::MapMode::Read, |_| () );
            gpu.device.poll(wgpu::Maintain::Wait);
            
            let data = buffer_slice.get_mapped_range().to_vec();
            
            self.buffers.insert(binding, BufferData::new(data));
        }
    }
}