use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
enum Wert {
    Ganzzahl(i32),
    Kommazahl(f32),
}

enum Operation {
    Plus,
    Minus,
    Mal,
    Geteilt,
}

trait Lösbar {
    fn lösung(self: Self) -> Wert;
}

impl Lösbar for Wert {
    fn lösung(self: Self) -> Wert {
        self
    }
}

impl Add<Wert> for Wert {
    type Output = Wert;
    fn add(self, rhs: Wert) -> Self::Output {
        match (self, rhs) {
            (Wert::Ganzzahl(x), Wert::Ganzzahl(y)) => Wert::Ganzzahl(x + y),
            (Wert::Kommazahl(x), Wert::Ganzzahl(y)) => Wert::Kommazahl(x + y as f32),
            (Wert::Ganzzahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x as f32 + y),
            (Wert::Kommazahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x + y),
        }
    }
}

impl Sub<Wert> for Wert {
    type Output = Wert;
    fn sub(self, rhs: Wert) -> Self::Output {
        match (self, rhs) {
            (Wert::Ganzzahl(x), Wert::Ganzzahl(y)) => Wert::Ganzzahl(x - y),
            (Wert::Kommazahl(x), Wert::Ganzzahl(y)) => Wert::Kommazahl(x - y as f32),
            (Wert::Ganzzahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x as f32 - y),
            (Wert::Kommazahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x - y),
        }
    }
}

impl Mul<Wert> for Wert {
    type Output = Wert;
    fn mul(self, rhs: Wert) -> Self::Output {
        match (self, rhs) {
            (Wert::Ganzzahl(x), Wert::Ganzzahl(y)) => Wert::Ganzzahl(x * y),
            (Wert::Kommazahl(x), Wert::Ganzzahl(y)) => Wert::Kommazahl(x * y as f32),
            (Wert::Ganzzahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x as f32 * y),
            (Wert::Kommazahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x * y),
        }
    }
}

impl Div<Wert> for Wert {
    type Output = Wert;
    fn div(self, rhs: Wert) -> Self::Output {
        match (self, rhs) {
            (Wert::Ganzzahl(x), Wert::Ganzzahl(y)) => Wert::Kommazahl(x as f32 / y as f32),
            (Wert::Kommazahl(x), Wert::Ganzzahl(y)) => Wert::Kommazahl(x / y as f32),
            (Wert::Ganzzahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x as f32 / y),
            (Wert::Kommazahl(x), Wert::Kommazahl(y)) => Wert::Kommazahl(x / y),
        }
    }
}

struct Rechnung<A: Lösbar, B: Lösbar> {
    links: A,
    rechts: B,
    operation: Operation,
}

impl<A: Lösbar, B: Lösbar> Lösbar for Rechnung<A, B> {
    fn lösung(self: Self) -> Wert {
        match self.operation {
            Operation::Plus => self.links.lösung() + self.rechts.lösung(),
            Operation::Minus => self.links.lösung() - self.rechts.lösung(),
            Operation::Mal => self.links.lösung() * self.rechts.lösung(),
            Operation::Geteilt => self.links.lösung() / self.rechts.lösung(),
        }
    }
}

fn main() {
    let unter_rechnung = Rechnung {
        links: Wert::Ganzzahl(1),
        operation: Operation::Plus,
        rechts: Wert::Ganzzahl(1),
    };

    let rechnung = Rechnung {
        links: unter_rechnung,
        operation: Operation::Geteilt,
        rechts: Wert::Ganzzahl(4),
    };

    println!("(1 + 1) / 4 = {:?}", rechnung.lösung())
}
