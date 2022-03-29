use super::{
    super::classes::{Polynomial, Term},
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

    // fn factor(&mut self) -> Box<AST> {
    //   let token = self.current_token.clone();
    //   let mut node: Box<AST>;
    //   if token.kind == TokenKind::Add {
    //     self.eat(TokenKind::Add);
    //     node = Box::new(AST::Unary(Unary {
    //       sign: Sign::Add,
    //       unary: self.factor(),
    //     }));
    //   } else if token.kind == TokenKind::Subtract {
    //     self.eat(TokenKind::Subtract);
    //     node = Box::new(AST::Unary(Unary {
    //       sign: Sign::Sub,
    //       unary: self.factor(),
    //     }));
    //   } else if token.kind == TokenKind::Number {
    //     self.eat(TokenKind::Number);
    //     node = Box::new(AST::Number(token.value.parse().unwrap()));
    //   } else if token.kind == TokenKind::LeftParen {
    //     self.eat(TokenKind::LeftParen);
    //     node = self.polynomial();
    //     self.eat(TokenKind::RightParen);
    //   } else {
    //     self.error(format!(
    //       "SyntaxError: Unexpected {:?}: '{}' at position {}:{}",
    //       self.current_token.kind.clone(),
    //       self.current_token.value.clone(),
    //       self.current_token.position.human.line.clone(),
    //       self.current_token.position.human.column.clone(),
    //     ));
    //   }
    //   if self.current_token.kind == TokenKind::Power {
    //     self.eat(TokenKind::Power);
    //     node = Box::new(AST::Index(Index {
    //       sign: Sign::Pow,
    //       index: (node, self.factor()),
    //     }));
    //   }

    //   node
    // }

    fn term(&mut self, negative: bool) -> Box<Term> {
        let mut coef = 1f64;
        if negative {
            coef = -1f64;
        }
        let mut exp = 0f64;
        let token = self.current_token.clone();
        if token.kind == TokenKind::Number {
            self.eat(TokenKind::Number);
            coef *= token.value.parse::<f64>().unwrap();
        }
        let token = self.current_token.clone();
        if token.kind == TokenKind::Identifier && token.value == "x" {
            self.eat(TokenKind::Identifier);
            exp = 1f64;
            if self.current_token.kind == TokenKind::Power {
                self.eat(TokenKind::Power);
                exp = self.current_token.value.parse::<f64>().unwrap();
                self.eat(TokenKind::Number);
            }
        }

        Term::new(coef, exp)
    }

    fn polynomial(&mut self) -> Box<Polynomial> {
        let mut node: Vec<Box<Term>> = vec![self.term(false)];

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
