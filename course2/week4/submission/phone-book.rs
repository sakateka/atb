use std::io;

const PRIME: usize = 10_000_019;

struct PhoneBook {
    store: Vec<(bool, usize, String)>,
    size: usize,
}

impl PhoneBook {
    fn new() -> Self {
        let size = usize::pow(10, 5);
        PhoneBook {
            store: vec![(false, 0, "not found".to_string()); size],
            size: size,
        }
    }

    fn find(&self, num: usize) -> String {
        let mut idx = self.hash(num);
        let start_idx = idx;
        loop {
            let (reserved, phone, ref name) = self.store[idx];
            if phone == num || !reserved {
                return name.to_string();
            }
            idx = (idx+1) % self.size;
            if idx == start_idx {
                return "not found".to_string();
            }
        }
    }

    fn add(&mut self, num: usize, name: String) {
        let mut idx = self.hash(num);
        let start_idx = idx;
        loop {
            let (_, phone, person) = self.store[idx].clone();
            if phone == num || person == "not found" {
                self.store[idx] = (true, num, name);
                break;
            }
            idx = (idx+1) % self.size;
            if idx == start_idx {
                panic!("phone book is full");
            }
        }
    }

    fn del(&mut self, num: usize) {
        let mut idx = self.hash(num);
        let start_idx = idx;
        loop {
            let (reserved, phone, _) = self.store[idx].clone();
            if phone == num || !reserved {
                self.store[idx] = (reserved, phone, "not found".to_string());
                break;
            }
            idx = (idx+1) % self.size;
            if idx == start_idx {
                break;
            }
        }
    }

    fn hash(&self, num: usize) -> usize {
        ((1*num + 2) % PRIME) % self.size
    }
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();

    let mut book = PhoneBook::new();

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        match &buf[0..2] {
            "fi" => { 
                let idx = buf[5..buf.len()-1].parse::<usize>().unwrap();
                println!("{}", book.find(idx));
            },
            "ad" => { 
                let sep = buf[4..].find(' ').unwrap();
                let idx = buf[4..4+sep].parse::<usize>().unwrap();
                let name = buf[4+1+sep..buf.len()-1].to_string();
                book.add(idx, name);
            },
            "de" => {
                let idx = buf[4..buf.len()-1].parse::<usize>().unwrap();
                book.del(idx);
            },
            _ => unreachable!(),
        }
        buf.clear();
    }
}
