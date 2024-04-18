use spirv_builder::SpirvBuilder;

pub fn build_shader(path: &str) -> Option<spirv_builder::SpirvBuilderError> {
    let shader = SpirvBuilder::new(path, "spirv-unknown-vulkan1.1")
    .build().expect("Error compiling kernel");
    // return shader.err()
    None
}

pub fn build_all(directory: &str) {
    
    for shader in std::fs::read_dir(directory)
    .expect(&format!("Error finding directory ({})", directory)) {
        let path = shader.expect(&format!("Invalid path in directory ({})", directory)).path();
        SpirvBuilder::new(path, "spirv-unknown-vulkan1.1")
            .build()
            .expect("Kernel failed to compile");
    }
}