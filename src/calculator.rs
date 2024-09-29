#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operators {
    Adition,
    Substruction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Operator(Operators),
    Bracket(char),
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    ParensMismatch,
}

pub struct Calculator {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for char in chars {
            match char {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (char as u32 - 48);
                    }
                    _ => {
                        let digit = char as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(char);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::ParensMismatch);
                        }
                    } else {
                        return Err(Error::ParensMismatch);
                    }
                }
                '+' => tokens.push(Token::Operator(Operators::Adition)),
                '-' => tokens.push(Token::Operator(Operators::Substruction)),
                '*' => tokens.push(Token::Operator(Operators::Multiplication)),
                '/' => tokens.push(Token::Operator(Operators::Division)),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(char)),
            }
        }

        if parens.len() > 0 {
            return Err(Error::ParensMismatch);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token>{
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop(){
            match token {
                Token::Number(_) => queue.push(token),
                Token::Operator(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token{
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('('){
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {},
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        queue
    }


    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32>{
        tokens.reverse();
        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match  token{
                Token::Number(num) => stack.push(num as f32),
                Token::Operator(Operators::Adition) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Operator(Operators::Substruction) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Operator(Operators::Multiplication) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Operator(Operators::Division) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },
                _ => {},
            }
        }

        if stack.len() > 1{
            None
        }else {
            stack.pop()
        }
    }
}
