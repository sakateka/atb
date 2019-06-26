use std::io;
use std::collections::VecDeque;

const PRIME: usize = 1_000_000_007;
const X: usize = 263;

struct ChainHashTable {
    store: Vec<VecDeque<String>>,
    size: usize,
}

impl ChainHashTable {
    fn new(m: usize) -> Self {
        ChainHashTable {
            store: vec![VecDeque::new(); m],
            size: m,
        }
    }

    fn find(&self, text: &String) -> bool {
        let idx = self.hash(text);
        self._find(idx, text)
    }

    fn _find(&self, idx: usize, text: &String) -> bool {
        self.store[idx].contains(text)
    }

    /*
    fn add(&mut self, text: String) {
        let idx = self.hash(&text);
        let present = self._find(idx, &text);
        if !present {
            self.store[idx].push(text);
        }
    }

    fn del(&mut self, text: String) {
        let idx = self.hash(&text);
        let some_pos = self.store[idx]
            .iter().position(|&ref t| t == &text);
        if let Some(pos) = some_pos {
            let last = self.store[idx].pop().unwrap();
            if last != text {
                assert_eq!(self.store[idx][pos], text);
                self.store[idx][pos] = last;
            }
        }
    }

    fn check(&self, bucket: usize) -> Vec<String> {
        self.store[bucket].clone()
    }

    */

    fn add(&mut self, text: String) {
        let idx = self.hash(&text);
        let present = self._find(idx, &text);
        if !present {
            self.store[idx].push_front(text);
        }
    }

    fn del(&mut self, text: String) {
        let idx = self.hash(&text);
        let some_pos = self.store[idx]
            .iter().position(|&ref t| t == &text);
        if let Some(pos) = some_pos {
            self.store[idx].remove(pos);
        }
    }

    fn check(&self, bucket: usize) -> VecDeque<String> {
        self.store[bucket].clone()
    }

    fn hash(&self, text: &str) -> usize {
        let mut hash = 0;
        for c in text.bytes().rev() {
            hash = (hash*X + c as usize) % PRIME
        }
        hash % self.size
    }
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_backets = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();

    let mut cht = ChainHashTable::new(num_backets);

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        match &buf[0..2] {
            "fi" => {
                let text = buf[5..].trim().to_string();
                if cht.find(&text) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            "ad" => {
                let text = buf[4..].trim().to_string();
                cht.add(text);
            },
            "de" => {
                let text = buf[4..].trim().to_string();
                cht.del(text);
            },
            "ch" => {
                let idx = buf[6..buf.len()-1]
                    .parse::<usize>().unwrap();
                buf.clear();
                for item in cht.check(idx).iter() {
                    buf.push_str(item);
                    buf.push(' ');
                }
                buf.pop();
                println!("{}", buf);
                buf.clear();
            },
            _ => unreachable!(),
        }
        buf.clear();
    }
}
