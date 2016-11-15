//! Error codes

pub struct ErrorType {
    num: u32,
    desc: &'static str
}
impl ErrorType {
    pub fn code(&self) -> u32 {
        self.num
    }
    pub fn desc(&self) -> &'static str {
        self.desc
    }
}
