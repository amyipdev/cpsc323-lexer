use core::fmt;
use std::{fmt::Debug, iter::Peekable, str::Chars, io::Write};

#[derive(Debug)]
enum States {
    Start,
    DefiningIdentifier,
    DefiningInteger,
    DefiningReal,
}

struct Token {
    pub ty: TokenType,
    pub lex: String,
}

#[derive(Debug)]
enum TokenType {
    Identifier,
    Number,
    Real,
    Separator,
    Operator,
    Keyword,
}

// If a LexerError is returned, the rest of the string should be considered void
#[derive(Debug, PartialEq)]
enum LexerError {
    IllegalDot,
    InternalStateError,
    InvalidIdentifier,
    Eof
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for LexerError {}

// This isn't a full C lexer by any means - we don't handle many types of tokens
// For instance, ident.ident is not handled here
// We also don't handle C string syntax (oh no)
// Howveer, there is a bit more functionality than is actually required for the input
fn lexer(chas: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut st = States::Start;
    let mut lexeme = String::new();
    loop {
        let c = *(match chas.peek() {
            Some(v) => v,
            None => match st {
                States::Start => return Err(LexerError::Eof),
                _ => return Ok(basic(st, lexeme)?)
            },
        });
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chas.next();
                match st {
                    States::Start => {}
                    _ => return Ok(basic(st, lexeme)?)
                }
            }
            '.' => match st {
                States::DefiningInteger => {
                    st = States::DefiningReal;
                    lexeme.push('.');
                    chas.next();
                }
                _ => return Err(LexerError::IllegalDot),
            },
            '(' | ')' | ';' => {
                match st {
                    States::Start => {
                        chas.next();
                        return Ok(Token {
                            ty: TokenType::Separator,
                            lex: c.to_string()
                        })
                    },
                    _ => return Ok(basic(st, lexeme)?)
                }
            }
            '<' | '>' | '=' => {
                match st {
                    States::Start => {
                        chas.next();
                        return Ok(Token {
                            ty: TokenType::Operator,
                            lex: c.to_string()
                        })
                    },
                    _ => return Ok(basic(st, lexeme)?)
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                chas.next();
                match st {
                    States::Start => st = States::DefiningInteger,
                    _ => {}
                }
                lexeme.push(c);
            }
            _ => {
                chas.next();
                lexeme.push(c);
                match st {
                    States::Start => st = States::DefiningIdentifier,
                    States::DefiningIdentifier => {},
                    _ => return Err(LexerError::InvalidIdentifier),
                }
            },
        };
    }
}

fn basic(st: States, lexeme: String) -> Result<Token, LexerError> {
    match st {
        States::Start => return Err(LexerError::InternalStateError),
        States::DefiningIdentifier => {
            match lexeme.as_str() {
                // expansion: add more keywords
                "while" => {
                    return Ok(Token {
                        ty: TokenType::Keyword,
                        lex: lexeme,
                    })
                }
                _ => {
                    return Ok(Token {
                        ty: TokenType::Identifier,
                        lex: lexeme,
                    })
                }
            }
        }
        States::DefiningInteger => {
            return Ok(Token {
                ty: TokenType::Number,
                lex: lexeme,
            })
        }
        States::DefiningReal => {
            return Ok(Token {
                ty: TokenType::Real,
                lex: lexeme,
            })
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = std::fs::read_to_string("input_scode.txt")?;
    let mut i = b.chars().peekable();
    let mut f = std::io::BufWriter::new(std::fs::File::create("output_file.txt")?);
    loop {
        let val = lexer(&mut i);
        match val {
            Ok(tok) => writeln!(f, "{:>10} = {}", format!("{:?}", tok.ty), tok.lex),
            Err(e) => {
                f.flush()?;
                if e == LexerError::Eof {
                    std::process::exit(0);
                } else {
                    eprintln!("Unacceptable error: {}", e);
                    std::process::exit(1);
                }
            }
        };
    }
}
