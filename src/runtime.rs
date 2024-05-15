pub fn build_runtime<S: AsRef<std::path::Path> + ?Sized>(path: &S) -> Result<Vec<u8>, spirv_builder::SpirvBuilderError> {

    let file = spirv_builder::SpirvBuilder::new(path, super::COMPILE_TARGET)
        .print_metadata(spirv_builder::MetadataPrintout::None)
        .build();
    if file.is_err() {
        return Err(file.err().unwrap());
    }
    
    Ok(std::fs::read(file.unwrap().module.unwrap_single()).unwrap())
}

pub fn build_runtime_multi<S: AsRef<std::path::Path> + ?Sized>(path: &S) -> Result<std::collections::HashMap<String, Vec<u8>>, spirv_builder::SpirvBuilderError> {
    let file = spirv_builder::SpirvBuilder::new(path, super::COMPILE_TARGET)
        .print_metadata(spirv_builder::MetadataPrintout::None)
        .build();
    
    if file.is_err() {
        return Err(file.err().unwrap());
    }
    
    // let mut res = vec![];

    
    let mut result = std::collections::HashMap::with_capacity(10);

    for (name, file) in file.unwrap().module.unwrap_multi().iter() {
        let bytes = std::fs::read(file).unwrap();
        result.insert(String::from(name), bytes);
    }

    Ok(result)
    // Ok(result)
}