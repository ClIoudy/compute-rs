use compute_rs::gpu::*;

// load "my_shader", the environment variable "my_shader.spv" is provided by the spirv_builder crate in build.rs 
// alternitavely, shaders can also be loaded at run time by enabling the feature "runtime_building", which, however, is very slow and and not advised
const SHADER: &[u8] = include_bytes!(env!("my_shader.spv"));

fn main() {

    // create gpu object
	let gpu = Gpu::new();

    // create kernel - see "my_shader" crate for more information
    let mut main_cs = gpu.new_kernel(SHADER, "main_cs");
    
    // create data for a buffer
    let data: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];	

    // create buffer from the data with binding (see my_shader/lib.rs for bindings)
	let mut buffer = Buffer::from(0, &data);


    println!("before dispatch: {:?}", buffer.data()[0]);

    // add the buffer to the kernel. This has to be done again after every dispatch
    main_cs.add_buffer(&mut buffer);

    // dispatch the shader, which mutates the buffer
    main_cs.dispatch(32, 1, 1);
    
    println!("after dispatch: {:?}", buffer.data()[0]);

    // buffer has to be added again before dispatch is possible
    main_cs.add_buffer(&mut buffer);
    main_cs.dispatch(32, 1, 1);

    println!("after another dispatch: {:?}", buffer.data()[0]);




    // another example, see my_shader/lib.rs kernel_2 for information on what's happening here
    let mut kernel_2 = gpu.new_kernel(SHADER, "kernel_2");

    let data = [0; 32];
    let multiplication_factor = 3;

    let mut data_buffer = Buffer::from(0, &data);
    let mut mult_buffer = Buffer::from(1, &multiplication_factor);

    kernel_2.add_buffer(&mut data_buffer);
    kernel_2.add_buffer(&mut mult_buffer);

    kernel_2.dispatch(32, 1, 1);

    println!("second example result: {:?}", data_buffer.data()[0]);

}