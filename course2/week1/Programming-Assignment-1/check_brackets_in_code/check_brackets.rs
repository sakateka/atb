use std::io;

struct Bracket(char, usize);

impl Bracket {
    fn check(&self, c: char) -> bool {
        match c {
            ']' => self.0 == '[',
            '}' => self.0 == '{',
            ')' => self.0 == '(',
            _ => false,
        }
    }
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();
    for c in buf.chars().enumerate() {
        let (idx, c) = c;
        if c == '(' || c == '[' || c == '{' {
            stack.push(Bracket(c, idx))
        }
        if c == ')' || c == ']' || c == '}' {
            let some_b = stack.pop();
            match some_b {
                Some(b) => if !b.check(c) {
                    println!("{}", idx+1);
                    return
                },
                None => {
                    println!("{}", idx+1);
                    return
                }

            }
        }
    }
    match stack.pop() {
        Some(b) => {
            println!("{}", b.1+1);
        }
        None => {
            println!("Success");
        }
    }
}
