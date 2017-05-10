
pub trait UniqueId {
    pub fn increment(&mut self);
    pub fn decrement(&mut self);
    pub fn next(&self) -> Self;
}
