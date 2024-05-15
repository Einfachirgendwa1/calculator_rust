trait Get {
    fn get(self) -> f32;
}

impl Get for f32 {
    fn get(self) -> f32 {
        self
    }
}

impl Get for i32 {
    fn get(self) -> f32 {
        self as f32
    }
}

#[allow(dead_code)]
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
    fn get(self) -> f32 {
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
    let rechnung = Rechnung {
        l: 1,
        r: Rechnung {
            l: 10,
            r: 2,
            o: Operation::Mal,
        },
        o: Operation::Geteilt,
    };

    println!("1 / (10 * 2) = {}", rechnung.get());
}
