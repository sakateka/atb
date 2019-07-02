use std::io;

const PRIME: usize = 1_000_000_007;
const X: usize = 263;

fn poly_hash(text: &[u8]) -> usize {
    let mut hash = 0;
    for c in text.iter().rev() {
        hash = (hash*X + *c as usize) % PRIME
    }
    hash
}

fn precompute_hashes(text: &[u8], pattern_len: usize) -> Vec<usize> {
    let text_len = text.len();
    let mut result = vec![0;text_len - pattern_len + 1];
    let s = &text[(text_len - pattern_len)..text_len];
    result[text_len - pattern_len] = poly_hash(s);
    let mut y = 1_usize;
    for _ in 0..pattern_len {
        y = (y * X) % PRIME;
    }
    for i in (0..(text_len - pattern_len)).rev() {
        let sub = y * (text[i+pattern_len] as usize) % PRIME;
        let add = text[i] as isize - sub as isize;
        // Beware of taking negative numbers (mod p).
        // In many programming languages, (-2)%5 != 3%5.
        // Thus you can compute the same hash values for two strings,
        // but when you compare them, they appear to be different.
        // To avoid this issue, you can use such construct in the code:
        // x = ((a%p) + p)%p instead of just x = a%p.
        let add = (add + PRIME as isize) as usize % PRIME;
        result[i] = (X * result[i+1] + add) % PRIME;
    }
    return result;
}

fn are_equal(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            return false;
        }
    }
    true
}

fn rabin_karp(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut result = Vec::new();
    let p_hash = poly_hash(pattern);
    let t_hashes = precompute_hashes(text, pattern.len());
    for i in 0..(text.len() - pattern.len() + 1) {
        if p_hash != t_hashes[i] {
            continue;
        }
        if are_equal(&text[i..(i+pattern.len())], pattern) {
            result.push(i);
        }
    }
    return result;
}


pub fn main() {
    let mut pattern = String::new();
    io::stdin().read_line(&mut pattern).unwrap();
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let result = rabin_karp(text.trim().as_bytes(), pattern.trim().as_bytes());

    for c in result.iter() {
        print!("{} ", c);
    }
    println!("");
}
