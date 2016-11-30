struct Parser {
    
}

trait Token {
    fn nud(&self) -> Option<Box<Token>> {
        None
    }
    fn led(&self) -> Option<Box<Token>> {
        None
    }
    fn lbp(&self) -> u8;
}

enum TokenResult {
    None,

}

struct Literal {
    pub ident: &'static str
}

impl Token for Literal {
    fn nud(&self) -> Option<Box<Token>> {
        None
    }
    fn lbp(&self) -> u8 { 10 }
}
