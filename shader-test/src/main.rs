#![allow(unused)]
use std::time::{Duration, Instant};

use compute_rs::gpu::*;

// load "my_shader", the environment variable "my_shader.spv" is provided by the spirv_builder crate in build.rs 
// alternitavely, shaders can also be loaded at run time by enabling the feature "runtime_building", which, however, is very slow and and not advised
const SHADER: &[u8] = include_bytes!(env!("my_shader.spv"));

fn main() {

	let gpu = Gpu::new();

    let mut shader = gpu.new_shader(SHADER);
    
    let array_buffer = {
        let data: [i32; 128] = (0..128).collect::<Vec<_>>().try_into().unwrap();
        let buffer = Buffer::new(&data, 0);
        shader.add_buffer(buffer)
    };
    
    shader.label("One");
    shader.freeze_layout(&gpu);

    let mut shader_2 = gpu.new_shader(SHADER);
    
    let array_buffer = {
        let data: [i32; 128] = (0..128).collect::<Vec<_>>().try_into().unwrap();
        let buffer = Buffer::new(&data, 0);
        shader.add_buffer(buffer)
    };
    
    shader_2.label("Two");
    shader_2.freeze_layout(&gpu);


    shader.dispatch("k", 8, 1, 1, &gpu);
    shader_2.dispatch("k", 8, 1, 1, &gpu);
    // println!("{:?}", shader.get(&array_buffer));
    

}