#[cfg(feature = "building")]
pub mod building;

#[cfg(feature = "runtime_building")]
pub mod runtime;

#[cfg(feature = "executor")]
pub mod gpu;

// use case:
// create buffer
// modify buffer
// add buffer -> get back buffer id
// freeze layout
// dispatch
// read back buffer contents via id (BufferId has data to type function for this)
// replace buffer (same as add buffer)

// !!!! Bindings (id) needed for buffers so that shader can access the right one

// make buffers HashMap in Shader?
// not nice: sizes vector
// if necessary: use hashmap instead