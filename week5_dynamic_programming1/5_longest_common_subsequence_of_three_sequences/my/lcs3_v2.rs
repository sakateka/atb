use std::io;

fn lcs2(source: &[i32], target: &[i32]) -> usize {
    let s_len = source.len();
    let t_len = target.len();

    let mut d: Vec<Vec<usize>> = vec![vec![0;t_len+1];s_len+1];
    for s in 1..(s_len+1) {
        for t in 1..(t_len+1) {
            let ins = d[s][t - 1];
            let del = d[s - 1][t];
            let mis = d[s - 1][t - 1];
            let mth = d[s - 1][t - 1] + 1;
            if source[s - 1] == target[t - 1] {
                d[s][t] = usize::max(mth, usize::max(ins, del));
            } else {
                d[s][t] = usize::max(mis, usize::max(ins, del));
            }
        }
    }
    d[s_len][t_len]
}

fn lcs3(seq1: &[i32], seq2: &[i32], seq3: &[i32]) -> usize {
    let lcs_1_2 = lcs2(seq1, seq2);
    let lcs_2_3 = lcs2(seq2, seq3);
    let lcs_1_3 = lcs2(seq1, seq3);
    usize::min(lcs_1_2, usize::min(lcs_2_3, lcs_1_3))
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();  // seq1 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq1: Vec<i32> = buf.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap(); // seq2 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq2: Vec<i32> = buf.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();  // seq3 len
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let seq3: Vec<i32> = buf.trim()
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

#[test]
fn lcs3_stress_test() {
    let lcs = lcs3(&[1, 2, 3], &[2, 1, 3], &[1, 3, 5]);
    assert_eq!(2, lcs);
}
