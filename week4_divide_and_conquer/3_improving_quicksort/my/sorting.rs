use std::io::{self, Read};

fn partition3(a: &mut [u32], left: usize, right: usize) -> (usize, usize) {
    let x = a[left];
    let mut j_left = left;
    let mut j_right = left;
    for i in (left+1)..(right+1) {
        if a[i] <= x {
            j_right += 1;
            let ai = a[i]; a[i] = a[j_right]; a[j_right] = ai;
            if ai < x {
                j_left += 1;
                a[j_right] = a[j_left]; a[j_left] = ai;
            }
        }
    }
    a[left] = a[j_left];
    a[j_left] = x;
    (j_left, j_right)
}

fn randomized_quick_sort(a: &mut [u32], left: usize, right: usize) {
    if left >= right {
        return
    }
    let k = left + (right - left)/2;
    let ak = a[k]; a[k] = a[left]; a[left] = ak;

    let (lm, rm) = partition3(a, left, right);
    if lm > 0 {
        randomized_quick_sort(a, left, lm - 1);
    }
    randomized_quick_sort(a, rm + 1, right);
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut numbers: Vec<u32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    let n = numbers[0] as usize;
    randomized_quick_sort(&mut numbers[1..], 0, n - 1);
    for i in numbers[1..].iter() {
        print!("{} ", i);
    }
    println!("");
}
