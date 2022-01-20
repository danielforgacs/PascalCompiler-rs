#[derive(Clone, Copy)]
enum Token {
    INTEGER(u32),
    PLUS,
    MINUS,
    EOF,
}

struct Interpreter {
    text: String,
    pos: usize,
    current_token: Token,
    current_char: char,
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
        let first_char = text.chars().nth(0).unwrap();
        Self {
            text,
            pos: 0,
            current_token: Token::EOF,
            current_char: first_char,
        }
    }

    fn error(&self) {
        panic!("Error parsing input.")
    }

    fn advanve(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = '\0';
        } else {
            self.current_char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    fn skip_white_space(&mut self) {
        while self.current_char.is_whitespace() {
            self.advanve();
        }
    }

    fn integer(&mut self) -> u32 {
        let mut result = String::new();
        while self.current_char.is_digit(10) {
            result.push(self.current_char);
            self.advanve();
        }
        result.parse::<u32>().unwrap()
    }

    fn get_next_token(&mut self) -> Token {
        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.skip_white_space();
                continue;
            }
            if self.current_char.is_digit(10) {
                return Token::INTEGER(self.integer());
            }
            if self.current_char == '+' {
                self.advanve();
                return Token::PLUS;
            }
            if self.current_char == '-' {
                self.advanve();
                return Token::MINUS;
            }
            self.error();
        }
        return Token::EOF;
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
        match op {
            Token::PLUS => self.eat(Token::PLUS),
            Token::MINUS => self.eat(Token::MINUS),
            _ => self.error(),
        }
        let right = &self.current_token.clone();
        self.eat(Token::INTEGER(0));
        match op {
            Token::PLUS => return left.int_value() + right.int_value(),
            Token::MINUS => return left.int_value() - right.int_value(),
            _ => {
                self.error();
                0
            },
        }
    }
}

fn main() {
    loop {
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

#[cfg(test)]
mod test {
    use super::Interpreter;

    #[test]
    fn interpreter() {
        let mut interpreter = Interpreter::new("345  +  432".to_string());
        assert_eq!(interpreter.expr(), 345  +  432);
    }

    #[test]
    fn interpreter_02() {
        let mut interpreter = Interpreter::new("123  -  73".to_string());
        assert_eq!(interpreter.expr(), 123  -  73);
    }
}
