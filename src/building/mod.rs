const COMPILE_TARGET: &str = "spirv-unknown-vulkan1.1";

#[cfg(feature = "building")]
pub mod runtime;

pub fn build_shader<S: AsRef<std::path::Path> + ?Sized>(path: &S) -> Result<spirv_builder::CompileResult, spirv_builder::SpirvBuilderError> {
    spirv_builder::SpirvBuilder::new(path.as_ref().to_owned(), COMPILE_TARGET)
    .build()
}

pub fn build_all<S: AsRef<std::path::Path> + ?Sized>(directory: &str) {
    for shader in std::fs::read_dir(directory)

    .expect(&format!("Error finding directory ({})", directory)) {
        let path = shader.expect(&format!("Invalid path in directory ({})", directory)).path();
        build_shader(&path).expect("Kernel failed to compile");
    }
}

