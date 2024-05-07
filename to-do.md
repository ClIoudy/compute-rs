- BufferRaw for buffer with phantomdata
- add attributes like read_only to bufferaw attributes
- clean up shader attributes/try to enhance performance by implementing bind_group attributes etc
- should shaders own buffers?
    - as in shader.create_buffer() -> &mut Buffer and the shader keeps the real buffer object
    - no mutable/immutable borrow issue








--------------------
- should shaders own buffers?
- as in shader.create_buffer() -> &mut Buffer and the shader keeps the real buffer object
- pro: easy handling ?
- con: what if one buffer used in multiple shaders sequentially  




-------------------------------------------------------------
- buffers contain wgpu buffers?
- dispatch should only have minimum amount of operations as necessary
    - move pipelinelayout to start of creation?
    - encoder will probably need to be created again every single dispatch
    - move bind group creation to seperate function for setting/initializing the bind group with all current buffers
        - shader creation should look like this: create shader, add buffers, initialize shader and then mostly leave it alone and just dispatch it
- some other optimisations maybe
- buffers 
    - more customizability 
    - different types of buffers (namely Texture buffers, uniforms)
- dispatch should work by creating staging buffers and sending them to the gpu and then copying back onto the actual buffers
- then update the buffer data
- always retrieve actual buffer data via function, which converts from &[u8] to T