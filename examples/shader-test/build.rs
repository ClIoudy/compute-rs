use compute_rs::building::build_shader;

fn main() {
    // build the my_shader crate as a spirv shader
    // this adds "my_shader.spv" as an environment variable which can be loaded at compile time
    build_shader("./my_shader").expect("Failed to build my_shader");
}