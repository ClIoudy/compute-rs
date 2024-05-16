#![no_std]

use spirv_std::spirv;
use spirv_std::glam::UVec3;

#[spirv(compute(threads(64)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [u32],
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] buffer_2: &mut [u32],
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] mul: &mut u32,
) {
    let index = id.x as usize;
    buffer[index] = buffer[index] * 3;
}

#[spirv(compute(threads(64)))]
pub fn kernel_2(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] multiplication_factor: &mut i32,
) {
    buffer[id.x as usize] = id.x * *multiplication_factor as u32;
}


