use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

trait Get {
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

struct Rechnung<A: Get, B: Get> {
    l: A,
    r: B,
    o: Operation,
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
        if stdin().read_line(&mut input).is_err() {
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

        let mut iter = rechnung.chars();
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
                let mut num = String::new();
                while let Some(buchstabe) = iter.next() {
                    // https://github.com/rust-lang/rust/issues/53667
                    if !buchstabe.is_digit(10) {
                        continue;
                    }

                    num.push(buchstabe);
                }
                tokens.push(Token::Literal(num.parse().unwrap()))
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

    dbg!(tokens);

    Err("Noch nicht implementiert".to_string())
}
