use std::io;

#[derive(Debug)]
struct Max {
    data: i32,
    count: u32,
}

#[derive(Debug)]
struct StackWithMax {
    stack: Vec<i32>,
    max: Vec<Max>,
}

impl StackWithMax {
    fn new() -> Self {
        StackWithMax {
            stack: Vec::new(),
            max: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let max = match self.max.last_mut() {
            Some(ref mut m) => {
                let mut new_max = None;
                if m.data == val {
                    m.count += 1;
                } else if m.data < val {
                    new_max = Some(Max{data: val, count: 1});
                }
                new_max
            },
            None => Some(Max{data: val, count: 1}),
        };
        if let Some(m) = max {
            self.max.push(m);
        }
        self.stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        let val = self.stack.pop().unwrap();
        let mut max = self.max.pop().unwrap();
        if max.data != val {
            self.max.push(max);
        } else if max.count > 1 {
            max.count -= 1;
            self.max.push(max);
        }
        val
    }

    fn max(&self) -> i32 {
        self.max.last().unwrap().data
    }
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    let mut stack = StackWithMax::new();
    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        match &buf[0..2] {
            "pu" => { 
                let num = &buf[5..buf.len() - 1]; // skip 'cmd ' and \n
                let val = num.parse::<i32>().unwrap();
                stack.push(val);
            },
            "po" => { stack.pop(); },
            "ma" => {
                println!("{:?}", stack.max());
            },
            _ => unreachable!(),
        }
        buf.clear();
    }
}
