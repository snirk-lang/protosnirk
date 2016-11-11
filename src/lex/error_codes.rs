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

macro_rules! error_codes {
    ( $($num:expr : $desc:expr),+ ) => {
        $(
            pub const conat!(ERR_, stringify!($num)): ErrorType
                = ErrorType { num: $num, desc: $desc };
         )*
    }
}
