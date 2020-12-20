#[macro_use]
extern crate lazy_static;

use regex::Match;
use regex::Regex;

enum Term {
    Number(u32),
    // Parentheses
    Expression(Box<Expression>)
}

enum Rhs {
    Term(Term),
    Expression(Box<Expression>)
}

enum Operation {
    Sum,
    Product
}

struct Expression {
    lhs: Term,
    op: Operation,
    rhs: Rhs
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Whitespace,
    Number(u32),
    Symbol(char),
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct Token<'a> {
    t: TokenType,
    info: Match<'a>
}

lazy_static!{
    static ref WHITESPACE_RE: Regex = Regex::new("^[ ]+").unwrap();
    static ref NUMBER_RE: Regex = Regex::new("^[0-9]+").unwrap();
    static ref SYMBOL_RE: Regex = Regex::new("^[+,*]").unwrap();
    static ref LPAREN_RE: Regex = Regex::new("^[(]").unwrap();
    static ref RPAREN_RE: Regex = Regex::new("^[)]").unwrap();
    static ref RULES_RE: Vec<&'static Regex> = vec![
        &WHITESPACE_RE, 
        &NUMBER_RE,
        &SYMBOL_RE,
        &LPAREN_RE,
        &RPAREN_RE,
    ];
}

fn get_token_type(i: usize, text: &str) -> TokenType {
    match i {
        1 => TokenType::Number(text.parse::<u32>().unwrap()),
        2 => TokenType::Symbol(text.chars().next().unwrap()),
        3 => TokenType::LParen,
        4 => TokenType::RParen,
        _ => TokenType::Whitespace,
    }
}

fn match_next_token(text: &str) -> Option<Token> {
    for (i, re) in RULES_RE.iter().enumerate() {
        match re.find(text) {
            Some(m) => {
                return Some(
                    Token{
                        t: get_token_type(i, &m.as_str()),
                        info: m
                    }
                );
            }
            _ => {}
        }
    }
    None

}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut rest = &input.trim()[..];
    let mut result = Vec::new();
    while let Some(token) = match_next_token(&rest) {
        rest = &rest[token.info.end()..];
        result.push(token);
    }
    result
}

fn match_symbol<'a, T>(buffer:&mut T) -> Result<Operation, &'static str> 
where 
    T: Iterator<Item=&'a Token<'a>>
{    
    if let Some(s) = buffer.next() {
        match s.t {
            TokenType::Symbol('*') => Ok(Operation::Product),
            TokenType::Symbol('+') => Ok(Operation::Sum),
            other => Err(
                //&format!("Unexpected token {:?} at line {}. Expected '<number>' or '('", other, s.info.start())[..]
                "Unexpected token"
            )
        }
    } else {
        Err("Unexpected end of input. A symbol was expected ('+' or '*')")
    }
}

fn match_term<'a, T>(buffer: &'a mut T) -> Result<Term, &'static str> 
where 
    T: Iterator<Item=&'a Token<'a>>
{    
    if let Some(token) = buffer.next() {
        match token.t {
            TokenType::Number(n) => Ok(Term::Number(n)),
            TokenType::LParen    => {
                let res = Term::Expression(Box::new(
                    match_expression(&mut buffer)?
                ));
                if let Some(token2) = buffer.next() {
                    match token2.t {
                        TokenType::RParen => {},
                        _ => return Err("Unexpected end of input. ')' expected")
                    }
                }
                Ok(res)
            }
            other => Err(
                //&format!("Unexpected token {:?} at line {}. Expected '<number>' or '('", other, token.info.start())[..]
                "Unexpected token"
            )
        }
    } else {
        Err("Unexpected end of input. A term was expected (number or parentheses)")
    }
}

fn match_rhs<'a, T>(buffer: &'a mut T) -> Result<Rhs, &'static str>
where 
    T: Iterator<Item=&'a Token<'a>>
{
    let term = match_term(&mut buffer)?;
    if let Some(token) = buffer.peekable().peek() {
        if let TokenType::Symbol(c) = token.t {
            return Ok(Rhs::Expression(Box::new(Expression{
                lhs: term,
                op: match_symbol(&mut buffer)?,
                rhs: match_rhs(&mut buffer)?
            })));
        }
    }
    Ok(Rhs::Term(term))
}

fn match_expression<'a, T>(buffer: &'a mut T) -> Result<Expression, &'static str>
where 
    T: Iterator<Item=&'a Token<'a>> 
{
    Ok(Expression{
        lhs: match_term(&mut buffer)?,
        op: match_symbol(&mut buffer)?,
        rhs: match_rhs(&mut buffer)?
    })
}

fn compute_term(term: &Term) -> Result<u32, &str> {
    match term {
        Term::Number(n) => Ok(*n),
        Term::Expression(e) => compute_expression(&e)
    }
}

fn compute_expression(expr: &Expression) -> Result<u32, &str> {
    let Expression{lhs, op, rhs} = expr;
    let lhs_val = compute_term(lhs)?;
    let rhs_val = match rhs {
        Rhs::Term(t)       => compute_term(t)?,
        Rhs::Expression(e) => compute_expression(e)?
    };
    match op {
        Operation::Product => Ok(lhs_val*rhs_val),
        Operation::Sum     => Ok(lhs_val+rhs_val),
    }
}

pub fn solve(input: &Vec<Token>) -> Result<u32, &'static str> {
    let iter = &mut input.iter();
    let expr = match_expression(iter)?;
    if let Some(t) = iter.next() {
        println!("Warning: Unexpected input after expression '{:?}' at col {}", t.t, t.info.start());
    }
    compute_expression(&expr)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_test() {
        let input = String::from("2 * 3 + (3 + 5)");
        let mut input = tokenize(&input);
        input.retain(|x| 
            x.t != TokenType::Whitespace
        );
        assert_eq!(1, solve(&input).unwrap());
    }
}