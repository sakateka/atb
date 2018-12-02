use std::io;

pub fn merge(mut left: Vec<i32>, mut right: Vec<i32>, mut inv: u32) -> (Vec<i32>, u32) {
    let mut ret = Vec::new();

    while left.len() > 0 && right.len() > 0 {
        if left[0] > right[0] {
            inv += left.len() as u32;
            ret.push(right.remove(0));
        } else {
            ret.push(left.remove(0));
        }
    }
    ret.append(&mut left);
    ret.append(&mut right);
    (ret, inv)
}

pub fn inversions(array: &[i32]) -> (Vec<i32>, u32) {
    let arr_len = array.len();
    if arr_len == 1 {
        return (vec![array[0]], 0)
    }

    let (l1, inv1) = inversions(&array[..arr_len/2]);
    let (l2, inv2) = inversions(&array[arr_len/2..]);

    merge(l1, l2, inv1+inv2)
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let numbers: Vec<i32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let (_, inv) = inversions(&numbers[..]);
    println!("{}", inv);
}

#[test]
fn test_run() {
    let case1 = vec![2, 3, 9, 2, 9];
    let (_, inv) = inversions(&case1[..]);
    assert!(inv == 2);
    let case2 = vec![9, 8, 7, 3, 2, 1];
    let (_, inv) = inversions(&case2[..]);
    assert!(inv == 15);
}
