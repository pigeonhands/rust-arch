use std::path::Path;
use std::fs;
use std::error::Error;

enum Token{
    Str(String),
    NewLine,
    Num(i32),
    Assign(char),
    BlockStart(char),
    BlockEnd(char),
    OpenBracket(char),
    CloseBracket(char),
    DoubleQuote(char),
    SingleQuote(char),
}


struct TokenList {
    tokens: Vec<Token>,
}

fn handle_str(s : String) -> Token {
     match s.parse::<i32>(){
        Ok(i) => Token::Num(i),
        Err(_) => Token::Str(s),
    }
}

impl TokenList{
    pub fn from_str(s : String) -> Result<TokenList, Box<Error>>{
        let mut chars = s.chars();
        let mut str_buf = String::new();
        let mut tl = TokenList{
            tokens: Vec::new(),
        };

        loop {
            let c : char;
            match chars.next(){
                None=>break,
                Some(_c) => c = _c,
            }
            if c.is_whitespace(){
                if c == 'n'{
                    tl.add(Token::NewLine);
                }
            }
            match c {
                '#' => {
                    if !str_buf.is_empty(){
                        tl.add(handle_str(str_buf));
                        str_buf = String::new();
                    }
                    loop { 
                        match chars.next(){
                            None => break,
                            Some(__c) => if __c == '\n'{
                                tl.add(Token::NewLine);
                                break;
                            },
                        }
                    }
                    continue;
                },
                '=' | '\n' | '#' => {
                    if !str_buf.is_empty(){
                        tl.add(handle_str(str_buf));
                        str_buf= String::new();
                    }
                },
                '{' => tl.add(Token::BlockStart(c)),
                '}' => tl.add(Token::BlockEnd(c)),
                '(' => tl.add(Token::OpenBracket(c)),
                ')' => tl.add(Token::CloseBracket(c)),
                '"' => tl.add(Token::DoubleQuote(c)),
                '\''=> tl.add(Token::SingleQuote(c)),
                _ => {},
            }
        }

        Ok(tl)
    }

    fn add(&mut self, t: Token){
        self.tokens.push(t);
    }
}