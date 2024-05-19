#[derive(Debug)]
enum Rechenoperation {
    Plus,
    Minus,
    Mal,
    Geteilt,
}

#[derive(Debug)]
enum Get {
    Wert {
        wert: f32,
    },

    Rechnung {
        links: Box<Get>,
        rechts: Box<Get>,
        operation: Rechenoperation,
    },

    Zelle {
        inhalt: Vec<Get>,
    },
}

impl Get {
    fn get(self) -> Result<f32, ()> {
        match self {
            Self::Wert { wert } => Ok(wert),
            Self::Rechnung {
                links,
                rechts,
                operation,
            } => match operation {
                Rechenoperation::Plus => Ok(links.get()? + rechts.get()?),
                Rechenoperation::Minus => Ok(links.get()? - rechts.get()?),
                Rechenoperation::Mal => Ok(links.get()? * rechts.get()?),
                Rechenoperation::Geteilt => Ok(links.get()? / rechts.get()?),
            },
            Self::Zelle { inhalt } => {
                let mut iter = inhalt.into_iter();
                while let Some(val) = iter.next() {
                    match val {
                        Self::Wert { wert } => {
                            if let Some(x) = iter.next() {
                                println!("In diesem Bereich nach der Zahl {} nichts mehr erwartet, {:#?} gefunden.", wert, x);
                                return Err(());
                            }
                        }
                        _ => todo!(),
                    }
                }
                todo!()
            }
        }
    }
}

impl From<i32> for Box<Get> {
    fn from(value: i32) -> Self {
        Box::new(Get::Wert { wert: value as f32 })
    }
}

impl From<f32> for Box<Get> {
    fn from(value: f32) -> Self {
        Box::new(Get::Wert { wert: value })
    }
}

#[test]
fn tests() {
    assert_eq!(
        Get::Rechnung {
            links: Box::new(Get::Rechnung {
                links: 1.into(),
                rechts: 5.into(),
                operation: Rechenoperation::Plus
            }),
            rechts: 6.into(),
            operation: Rechenoperation::Mal
        }
        .get()
        .unwrap(),
        36 as f32
    )
}

fn main() {}
