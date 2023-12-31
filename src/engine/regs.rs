#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub data: [u64; 60],
}

impl core::ops::Index<usize> for Registers {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl core::ops::IndexMut<usize> for Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl Default for Registers {
    fn default() -> Self {
        Self { data: [0; 60] }
    }
}
