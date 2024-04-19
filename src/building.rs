use spirv_builder::SpirvBuilder;

pub fn build_shader(path: &str) -> Result<spirv_builder::CompileResult, spirv_builder::SpirvBuilderError> {
    SpirvBuilder::new(path, "spirv-unknown-vulkan1.1")
    .build()
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