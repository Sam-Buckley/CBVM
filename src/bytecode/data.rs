
pub trait ByteData {
    fn get(&self) -> u8;
    fn set(&mut self, data: u8);
}
impl std::fmt::Debug for dyn ByteData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ByteData")
    }
}
impl Clone for Box<dyn ByteData> {
    #[allow(unconditional_recursion)]
    fn clone(&self) -> Self {
        return Box::new((*self).clone())
    }
}

#[allow(unconditional_recursion)]
impl ByteData for Box<dyn ByteData> {
    fn get(&self) -> u8 {
        return (*self).get()
    }
    fn set(&mut self, data: u8) {
        return (*self).set(data)
    }
}