pub struct BufferRaw<'a> {
    pub binding: u32,
    pub data: &'a [u8],
    pub is_read_only: bool,
    pub has_dynamic_offset: bool,
    // pub is_uniform: bool,
}

impl BufferRaw<'_> {
    pub fn new(binding: u32) -> Self {
        Self {
            binding,
            ..Default::default()
        }
    }

    pub fn from_data(binding: u32, data: &'static [u8]) -> Self {
        Self {
            binding,
            data,
            ..Default::default()
        }
    }

    pub fn set_read_only(&mut self, set: bool) {
        self.is_read_only = set;
    } 

    pub fn set_dynamic_offset(&mut self, set: bool) {
        self.has_dynamic_offset = set;
    }
}

impl Default for BufferRaw<'_> {
    fn default() -> Self {
        Self {
            binding: 0,
            data: &[],
            is_read_only: false,
            has_dynamic_offset: false,
        }
    }
}