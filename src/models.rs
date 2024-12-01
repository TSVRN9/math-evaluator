use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Num(f64),
    Op(Op),
    LPar,
    RPar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Op {
    pub fn identity(&self) -> f64 {
        match self {
            Op::Add | Op::Sub => 0.0,
            Op::Mul | Op::Div | Op::Exp => 1.0,
        }
    }

    pub fn eval(&self, left: f64, right: f64) -> f64 {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
            Op::Exp => left.powf(right),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Term(Box<Expr>, Op, Box<Expr>),
    Literal(f64),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Term(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Expr::Literal(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Exp => "^",
        };
        write!(f, "{}", op_str)
    }
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Term(left, op, right) => {
                let left_val = left.eval();
                let right_val = right.eval();
                op.eval(left_val, right_val)
            }
            Expr::Literal(val) => *val,
        }
    }
}

pub struct Ast {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Self {
        Ast {
            tokens,
            current_index: 0,
        }
    }

    pub fn build(mut self) -> Box<Expr> {
        self.az()
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }

    fn eat(&mut self) -> &Token {
        let token = &self.tokens[self.current_index];
        self.current_index += 1;
        token
    }

    fn eat_op(&mut self) -> Op {
        match self.eat() {
            Token::Op(op) => op.clone(),
            _ => panic!("Current token is not an operator"),
        }
    }

    fn az(&mut self) -> Box<Expr> {
        // Based off PEMDAS
        //
        // az: md ((Add | Sub) md)*
        // md: e ((Mul | Div) e)*
        // e: p (Exp p)*
        // p: Num | LPar az RPar

        let mut expr = self.md();

        while matches!(
            self.current_token(),
            Some(Token::Op(Op::Add)) | Some(Token::Op(Op::Sub))
        ) {
            expr = Box::new(Expr::Term(expr, self.eat_op(), self.md()));
        }

        expr
    }

    fn md(&mut self) -> Box<Expr> {
        // md: e ((Mul | Div) e)*

        let mut expr = self.e();

        while matches!(
            self.current_token(),
            Some(Token::Op(Op::Mul)) | Some(Token::Op(Op::Div))
        ) {
            expr = Box::new(Expr::Term(expr, self.eat_op(), self.e()));
        }

        expr
    }

    fn e(&mut self) -> Box<Expr> {
        // e: p (Exp p)*

        let mut expr = self.p();

        while matches!(self.current_token(), Some(Token::Op(Op::Exp))) {
            expr = Box::new(Expr::Term(expr, self.eat_op(), self.p()));
        }

        expr
    }

    fn p(&mut self) -> Box<Expr> {
        // p: Num | LPar az RPar
        if matches!(self.current_token(), Some(Token::Num(_))) {
            if let &Token::Num(value) = self.eat() {
                return Box::new(Expr::Literal(value));
            } else {
                panic!("Not a literal?");
            }
        } else {
            // consume parens
            self.eat();
            let expr = self.az();
            self.eat();

            expr
        }
    }
}
