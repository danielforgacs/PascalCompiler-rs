#[derive(Clone, Copy)]
enum Token {
    INTEGER(u32),
    PLUS,
    EOF,
}

struct Interpreter {
    text: String,
    pos: usize,
    current_token: Token,
}

impl Token {
    fn int_value(&self) -> u32 {
        match self {
            Self::INTEGER(value) => *value,
            _ => panic!("no value for token type"),
        }
    }
}

impl Interpreter {
    fn new(text: String) -> Self {
        Self {
            text,
            pos: 0,
            current_token: Token::EOF,
        }
    }

    fn error(&self) {
        panic!("Error parsing input.")
    }

    fn get_next_token(&mut self) -> Token {
        let text = self.text.as_str();
        if self.pos > text.len() - 1 {
            return Token::EOF;
        }
        let current_char = text.chars()
            .nth(self.pos)
            .unwrap();
        match current_char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let token = Token::INTEGER(current_char.to_digit(10).unwrap());
                self.pos += 1;
                return token;
            }
            '+' => {
                let token = Token::PLUS;
                self.pos += 1;
                return token;
            }
            '\n' => {
                let token = Token::EOF;
                self.pos += 1;
                return token;
            }
            _ => panic!(),
        }
    }

    fn eat(&mut self, ttype: Token) {
        match &self.current_token {
            ttype => self.current_token = self.get_next_token(),
        }
    }

    fn expr(&mut self) -> u32 {
        self.current_token = self.get_next_token();
        let left = &self.current_token.clone();
        self.eat(Token::INTEGER(0));
        let op = &self.current_token.clone();
        self.eat(Token::PLUS);
        let right = &self.current_token.clone();
        self.eat(Token::INTEGER(0));
        left.int_value() + right.int_value()
    }
}

fn main() {
    loop {
        // let source = r#"5+2"#;
        let mut source = String::new();
        std::io::stdin().read_line(&mut source).unwrap();
        if source == "\n".to_string() {
            println!("Good bye.");
            break;
        }
        let mut interpreter = Interpreter::new(source.to_string());
        let result = interpreter.expr();
        println!("{}", result);
    }
}
