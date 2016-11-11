//! The tokenizer creates a stream of tokens for the parsers to turn into expressions

/// Generates a stream of tokens
pub struct Tokenizer<'a> {
    input: &['a str],
    index: usize,
    peeked: Option<Token<'a>>
}

impl Tokenizer {
    /// Create a new tokenizer using the given input text
    pub fn new<'a>(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input: input,
            index: 0
        }
    }

    pub fn peek() -> Token<'a> {
        let peeked = self.next().expect("Tokenizer never fails");
    }

    fn parse_next(&mut self) -> Option<Token<'a>> {
        
    }
}

impl<'a> Iterator<Item=Token<'a>> for Tokenizer<'a> {
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peeked) = self.peeked {
            self.peeked = None;
            Some(peeked)
        } else {
            
        }
    }
}
