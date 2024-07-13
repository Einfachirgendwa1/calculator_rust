enum Value<'a> {
    Literal(f64),
    Operation(&'a Operation<'a>),
}

enum Operation<'a> {
    Add(Value<'a>, Value<'a>),
    Sub(Value<'a>, Value<'a>),
    Mul(Value<'a>, Value<'a>),
    Div(Value<'a>, Value<'a>),
}

impl<'a> Value<'a> {
    fn solve(&self) -> f64 {
        match self {
            Self::Literal(x) => *x,
            Self::Operation(x) => x.solve(),
        }
    }
}

impl<'a> Operation<'a> {
    fn solve(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a.solve() + b.solve(),
            Operation::Sub(a, b) => a.solve() - b.solve(),
            Operation::Mul(a, b) => a.solve() * b.solve(),
            Operation::Div(a, b) => a.solve() / b.solve(),
        }
    }
}

fn main() {
    println!(
        "{}",
        Operation::Add(
            Value::Literal(1.),
            Value::Operation(&Operation::Mul(Value::Literal(3.), Value::Literal(5.)))
        )
        .solve()
    )
}
