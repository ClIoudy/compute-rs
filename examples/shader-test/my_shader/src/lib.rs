#![no_std]

use spirv_std::spirv;
use spirv_std::glam::UVec3;


#[spirv(compute(threads(8)))]
pub fn kernel_2(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [i32],
) {
    buffer[id.x as usize] *= 2;
}


#[spirv(compute(threads(8)))]
pub fn k(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [i32],
) {
    buffer[id.x as usize] *= 2;
}