pub struct BufferRaw {
    pub binding: u32,
    pub data: &'_ [u8],
    pub is_read_only: bool,
    pub has_dynamic_offset: bool,
    // pub is_uniform: bool,
}

impl BufferRaw {
    pub fn new(binding: u32) -> Self {
        Self {
            binding,
            ..Default::default(),
        }
    }

    pub fn from_data(binding: u32, data: &[u8]) {
        Self {
            binding,
            data,
            ..Default::default(),
        }
    }

    pub fn set_read_only(&mut self, bool: set) {
        self.is_read_only = set;
    } 

    pub fn set_dynamic_offset(&mut self, bool: set) {
        self.has_dynamic_offset = set;
    }
}

impl Default for BufferRaw {
    fn default() -> Self {
        Self {
            binding: 0,
            data: &[],
            is_read_only: false,
            has_dynamic_offset: false,
        }
    }
}