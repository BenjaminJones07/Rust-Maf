use super::{
    super::classes::{Consts, Maf, Polynomial, Term, Values, Vars},
    Lexer, Token, TokenKind,
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    // method to call for lexing errors
    fn error(&self, msg: String) -> ! {
        panic!("{}", msg)
    }

    fn eat(&mut self, token_type: TokenKind) {
        if self.current_token.kind == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            self.error(format!(
                "SyntaxError: Unexpected {:?}: '{}' at position {}:{} expecting {:?}",
                self.current_token.kind.clone(),
                self.current_token.value.clone(),
                self.current_token.position.human.line.clone(),
                self.current_token.position.human.column.clone(),
                token_type,
            ));
        }
    }

    fn term(&mut self, negative: bool) -> Box<Term> {
        let mut coef = 1f64;
        if negative {
            coef = -1f64;
        }
        let mut exp = 0f64;
        let mut vf: Vec<Box<dyn Maf>> = vec![];
        let token = self.current_token.clone();
        if token.kind == TokenKind::Number {
            self.eat(TokenKind::Number);
            coef *= token.value.parse::<f64>().unwrap();
        }
        let mut token = self.current_token.clone();
        while token.kind == TokenKind::Identifier {
            if Vars::names().contains(&token.value) {
                self.eat(TokenKind::Identifier);
                exp = 1f64;
                vf.push(Box::new(Values::Variable(Vars::from(token.value))));
                if self.current_token.kind == TokenKind::Power {
                    self.eat(TokenKind::Power);
                    exp = self.current_token.value.parse::<f64>().unwrap();
                    self.eat(TokenKind::Number);
                }
            } else if Consts::names().contains(&token.value) {
                println!("constant: {}", token.value);
                self.eat(TokenKind::Identifier);
                exp = 1f64;
                vf.push(Box::new(Values::Constant(Consts::from(token.value))));
                if self.current_token.kind == TokenKind::Power {
                    self.eat(TokenKind::Power);
                    exp = self.current_token.value.parse::<f64>().unwrap();
                    self.eat(TokenKind::Number);
                }
                // TODO: Something with consts
            } else if [
                "sin".to_string(),
                "cos".to_string(),
                "tan".to_string(),
                "cot".to_string(),
                "sec".to_string(),
                "csc".to_string(),
            ]
            .contains(&token.value)
                && self.lexer.peek().kind == TokenKind::LeftParen
            {
                println!("function: {}", token.value);
                self.eat(TokenKind::Identifier);
                exp = 1f64;
                if self.current_token.kind == TokenKind::Power {
                    self.eat(TokenKind::Power);
                    exp = self.current_token.value.parse::<f64>().unwrap();
                    self.eat(TokenKind::Number);
                }
                self.eat(TokenKind::LeftParen);
                self.eat(TokenKind::RightParen);
                // TODO: Something with functions
            }
            token = self.current_token.clone();
        }

        Term::new(coef, vf, exp)
    }

    fn polynomial(&mut self) -> Box<Polynomial> {
        let mut node: Vec<Box<dyn Maf>> = vec![self.term(false)];

        while [TokenKind::Add, TokenKind::Subtract].contains(&self.current_token.kind) {
            let token = self.current_token.clone();
            if token.kind == TokenKind::Add {
                self.eat(TokenKind::Add);
                node.push(self.term(false));
            } else if token.kind == TokenKind::Subtract {
                self.eat(TokenKind::Subtract);
                node.push(self.term(true));
            }
        }

        Polynomial::new(node)
    }

    pub fn parse(&mut self) -> Box<Polynomial> {
        self.polynomial()
    }
}

pub fn parse(input: String) -> Box<Polynomial> {
    let mut parser = Parser::new(input);
    parser.parse()
}
