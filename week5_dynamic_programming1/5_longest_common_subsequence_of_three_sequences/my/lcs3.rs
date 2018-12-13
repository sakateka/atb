use std::io;

fn lcs_matrix(source: &[i32], target: &[i32]) -> Vec<Vec<usize>> {
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
    d
}

fn lcs3(seq1: &[i32], seq2: &[i32], seq3: &[i32]) -> usize {
    let lcs_1_2 = get_lcs(seq1, seq2);
    let d = lcs_matrix(&lcs_1_2[..], seq3);
    d[lcs_1_2.len()][seq3.len()]
}

fn get_lcs(source: &[i32], target: &[i32]) -> Vec<i32> {
    let d = lcs_matrix(source, target);

    let mut s = source.len();
    let mut t = target.len();

    let mut lcs = Vec::new();

    while s != 0 && t != 0 {
        if s > 0 && d[s][t] == d[s - 1][t] {
            s -= 1;
        } else if t > 0 && d[s][t] == d[s][t - 1] {
            t -= 1;
        } else {
            let si = source[s-1];
            let ti = target[t-1];
            if si ==  ti {
                lcs.push(si);
            }
            s -= 1;
            t -= 1;
        }
    }
    lcs.reverse();
    println!("{:?}", lcs);
    lcs
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
