use std::io;

fn lcs3(seq1: &[i32], seq2: &[i32], seq3: &[i32]) -> usize {
    let l1 = seq1.len();
    let l2 = seq2.len();
    let l3 = seq3.len();

    let mut d: Vec<Vec<Vec<usize>>> = vec![vec![vec![0;l3+1];l2+1];l1+1];
    for s1 in 1..(l1+1) {
        for s2 in 1..(l2+1) {
            for s3 in 1..(l3+1) {
                let c1 = d[s1 - 1][s2][s3];
                let c2 = d[s1][s2 - 1][s3];
                let c3 = d[s1][s2][s3 - 1];
                if seq1[s1 - 1] == seq2[s2 - 1] && seq2[s2 - 1] == seq3[s3 - 1] {
                    d[s1][s2][s3] = d[s1 - 1][s2 - 1][s3 - 1] + 1;
                } else {
                    d[s1][s2][s3] = usize::max(c1, usize::max(c2, c3));
                }
            }
        }
    }
    d[l1][l2][l3]
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();  // seq1 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq1: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap(); // seq2 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq2: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();  // seq3 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq3: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let lcs = lcs3(&seq1[..], &seq2[..], &seq3[..]);
    println!("{}", lcs);
}

#[test]
fn lcs3_test() {
    let lcs = lcs3(&[1, 2, 3], &[2, 1, 3], &[1, 3, 5]);
    assert_eq!(2, lcs);

    let lcs = lcs3(&[1, 2, 3], &[1, 3, 2], &[2, 1, 3]);
    assert_eq!(2, lcs);

    let lcs = lcs3(&[8, 3, 2, 1, 7], &[8, 2, 1, 3, 8, 10, 7], &[6, 8, 3, 1, 4, 7]);
    assert_eq!(3, lcs);
}
