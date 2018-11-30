// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn default() -> Builder {
        Builder{ string: None, number: None }
    }

    fn string(&self, s: &str) -> Builder {
        Builder { string: Some(String::from(s)), ..*self }
    }

    fn number(&self, n: usize) -> Builder {
        Builder { number: Some(n), ..*self }
    }
}

impl ToString for Builder {
    fn to_string(&self) -> String {
        match (self.string, self.number) {
            (Some(s), Some(n)) => format!("{} {}", s, n),
            (Some(s), None) => s,
            (None, Some(n)) => format!("{}", n),
            (None, None) => String::from("")
        }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
