use std::collections::HashMap;

mod buffer;
use buffer::*;

mod dispatch;
mod layout;

pub use buffer::{Buffer, BufferId};

mod labels;

type BoundBuffer = (wgpu::Buffer, u32);

pub struct Shader {
    shader_label: Option<String>,
    buffers: HashMap<u32, BufferData>,
    staging_buffers: Vec<BoundBuffer>,
    bind_group: Option<wgpu::BindGroup>,
    pipeline_layout: Option<wgpu::PipelineLayout>,
    module: wgpu::ShaderModule,
    is_ready: bool,
}

impl Shader {
    pub fn new(module: wgpu::ShaderModule) -> Self {
        Self {
            shader_label: None,
            buffers: HashMap::new(),
            staging_buffers: vec![],
            bind_group: None,
            pipeline_layout: None,
            module,
            is_ready: false,
        }
    }

    pub fn get<T: Clone>(&self, id: &BufferId<T>) -> T {
        let x = self.buffers.get(&id.binding).unwrap();
        u8_as_any(&x.data)
    }

    pub fn get_checked<T: Clone>(&self, id: &BufferId<T>) -> Option<T> {
        if let Some(x) = self.buffers.get(&id.binding) {
            Some(u8_as_any(&x.data))
        } else {
            None
        }
    }

    pub fn is_already_bound<T: Clone>(&self, id: BufferId<T>) -> bool {
        self.buffers.contains_key(&id.binding)
    }

    pub fn add_buffer<T: Clone>(&mut self, buffer: Buffer<T>) -> BufferId<T> {
        let (data, id) = buffer.consume();

        self.buffers.insert(id.binding, data);

        id
    }

    pub fn label(&mut self, set_to: &str) {
        self.shader_label = Some(set_to.to_owned());
    }

}

// add buffer   ->  save buffer data

// freeze       ->  create bind group entries from buffers
//                  create bind group layout from entries
//                  create pipeline layout
//                  create pipeline

// dispatch     ->  dispatch

// bind group layout entry
// bind group layout
// bind group
// pipeline layout
// compute pipeline
// encoder
// cpass
// dispatch
