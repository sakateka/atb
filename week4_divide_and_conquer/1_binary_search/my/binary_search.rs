use std::io;

pub fn binary_search(a: &Vec<u32>, x: u32) -> isize {
    let mut left: isize = 0;
    let mut right = (a.len() - 1) as isize;

    while left <= right {
        let mid = left + (right - left)/2;
        if a[mid as usize] == x {
            return mid
        } else if a[mid as usize] < x {
            left = mid + 1
        } else {
            right = mid - 1
        }

    }
    return -1
}

pub fn liner_search(a: &Vec<u32>, x: u32) -> isize {
    for i in 0..a.len() {
        if a[i] == x {
            return i as isize
        }
    }
    return -1

}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let numbers: Vec<u32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).skip(1).collect();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let search: Vec<u32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).skip(1).collect();

    for i in search {
        let idx = binary_search(&numbers, i);
        print!("{} ", idx);
    }
    println!("")
}

#[test]
fn test_liner_search() {
    let numbers = vec![1, 5, 8, 12, 13];
    let search = vec![8, 1, 23, 1, 11];
    let results = vec![2, 0, -1, 0, -1];

    for i in 0..search.len() {
        let expect = results[i];
        assert_eq!(expect, liner_search(&numbers, search[i]));
    }
}

#[test]
fn test_binary_search() {
    let numbers = vec![1, 5, 8, 12, 13];
    let search = vec![8, 1, 23, 1, 11];

    for i in 0..search.len() {
        let expect = liner_search(&numbers, search[i]);
        assert_eq!(expect, binary_search(&numbers, search[i]));
    }
}
