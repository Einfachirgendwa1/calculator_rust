use std::{
    any::{Any, TypeId},
    fmt::{Display, Write as FmtWrite},
    io::{stdin, stdout, Write},
    process::exit,
};

trait Get: Display {
    fn get(&self) -> f32;
}

impl Get for f32 {
    fn get(&self) -> f32 {
        self.clone()
    }
}

impl Get for i32 {
    fn get(&self) -> f32 {
        self.clone() as f32
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Operation {
    Plus,
    Minus,
    Mal,
    Geteilt,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match *self {
            Operation::Plus => '+',
            Operation::Minus => '-',
            Operation::Mal => '*',
            Operation::Geteilt => '/',
        })
    }
}

struct Rechnung<A: Get, B: Get> {
    l: A,
    r: B,
    o: Operation,
}

impl<A: Get, B: Get> Display for Rechnung<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.l, self.o, self.r))
    }
}

impl<A: Get, B: Get> Get for Rechnung<A, B> {
    fn get(&self) -> f32 {
        let (l, r) = (self.l.get(), self.r.get());
        match self.o {
            Operation::Plus => l + r,
            Operation::Minus => l - r,
            Operation::Mal => l * r,
            Operation::Geteilt => l / r,
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        if stdin().read_line(&mut input).is_err() || {
            input = input.trim().to_string();
            input.is_empty()
        } {
            println!();
            exit(0);
        }

        match solve(&input) {
            Ok(value) => println!("{}", value),
            Err(message) => println!("Fehler: {}", message),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Literal(f32),
    Operation(Operation),
}

fn solve(rechnung: &str) -> Result<f32, String> {
    let tokens: Vec<Token> = match {
        let result: Result<Vec<Token>, String>;
        let mut tokens: Vec<Token> = Vec::new();

        let mut iter = rechnung.chars().peekable();
        while let Some(buchstabe) = iter.next() {
            if buchstabe.is_ascii_whitespace() {
                continue;
            }

            if match buchstabe {
                '+' => Some(tokens.push(Token::Operation(Operation::Plus))),
                '-' => Some(tokens.push(Token::Operation(Operation::Minus))),
                '*' => Some(tokens.push(Token::Operation(Operation::Mal))),
                '/' => Some(tokens.push(Token::Operation(Operation::Geteilt))),
                _ => None,
            }
            .is_some()
            {
                continue;
            }

            if buchstabe.is_digit(10) {
                let mut num = String::from(buchstabe);
                while let Some(buchstabe) = iter.peek() {
                    if !buchstabe.is_digit(10) {
                        break;
                    }

                    num.push(*buchstabe);
                    iter.next();
                }
                tokens.push(Token::Literal(num.parse().unwrap()));
                continue;
            }

            return Err(format!(
                "Variablen noch nicht implementiert (wegen Buchstabe '{}')",
                buchstabe
            ));
        }

        result = Ok(tokens);
        result
    } {
        Ok(value) => value,
        Err(message) => return Err(message),
    };

    let generic_structure: Vec<Box<dyn Get>> = match {
        let mut result: Vec<Box<dyn Get>> = Vec::new();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            match token {
                Token::Literal(x) => result.push(Box::new(x)),
                Token::Operation(x) => match result.pop() {
                    None => {
                        return Err(
                            "Links von dem Rechenzeichen wurde eine Zahl erwartet".to_string()
                        )
                    }
                    Some(y) => {
                        if !(y.type_id() == TypeId::of::<f32>()) {
                            return Err(format!("Links von dem Rechenzeichen {} wurde eine Zahl erwartet, nicht {}.", x, y));
                        } else {
                        }
                    }
                },
            }
        }

        Ok(result)
    } {
        Ok(value) => value,
        Err(message) => return Err(message),
    };

    Err("Noch nicht implementiert".to_string())
}
