pub mod models;

use models::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval(s: &str) -> Option<f64> {
    let tokens = tokenize(s);

    if let Some(tokens) = tokens {
        let ast = Ast::new(tokens).build();

        let value = ast.eval();

        Some(value)
    } else {
        None
    }
}

pub fn tokenize(s: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut buffer = String::new();

    for c in s.chars() {
        if !buffer.is_empty() && !c.is_digit(10) && c != '.' {
            let val = buffer.parse::<f64>().unwrap();
            tokens.push(Token::Num(val));
            buffer.clear();
        }

        match c {
            '.' if buffer.contains('.') => return None,
            '0'..='9' | '.' => {
                buffer.push(c);
            }
            '+' => tokens.push(Token::Op(Op::Add)),
            '-' => tokens.push(Token::Op(Op::Sub)),
            '*' => tokens.push(Token::Op(Op::Mul)),
            '/' => tokens.push(Token::Op(Op::Div)),
            '^' => tokens.push(Token::Op(Op::Exp)),
            '(' => tokens.push(Token::LPar),
            ')' => tokens.push(Token::RPar),
            ' ' | '\n' => {}
            _ => {
                return None;
            }
        }
    }

    if !buffer.is_empty() {
        let val = buffer.parse::<f64>().unwrap();
        tokens.push(Token::Num(val));
        buffer.clear();
    }

    let tokens = fix_tokens(tokens);

    tokens
}

fn fix_tokens(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
    let mut open_parens = 0; // count open and closed parentheses

    for t in tokens.iter() {
        match t {
            Token::LPar => {
                open_parens += 1;
            }
            Token::RPar => {
                open_parens -= 1;
            }
            _ => {}
        }

        if open_parens < 0 {
            // cannot be fixed
            return None;
        }
    }

    // add missing closing parens
    for _ in 0..open_parens {
        tokens.push(Token::RPar)
    }

    // add any default values
    for i in (0..tokens.len()).rev() {
        let look_ahead = tokens.get(i + 1).cloned();
        let current_token = tokens[i].clone();
        let look_behind = tokens.get(i.wrapping_sub(1)).cloned();

        if matches!(
            (&look_behind, &current_token),
            (Some(Token::RPar), Token::Num(_)) | (Some(Token::RPar), Token::LPar)
        ) {
            tokens.insert(i, Token::Op(Op::Mul));
        }

        if let Token::Op(op) = current_token {
            if !matches!(look_ahead, Some(Token::Num(_))) {
                tokens.insert(i + 1, Token::Num(op.identity()));
            }

            if !matches!(look_behind, Some(Token::Num(_) | Token::RPar)) {
                tokens.insert(i, Token::Num(op.identity()));
            }
        }
    }

    Some(tokens)
}
