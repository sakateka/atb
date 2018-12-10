use std::io;

fn lcs2(source: &[i32], target: &[i32]) -> usize {
    let s_len = source.len();
    let t_len = target.len();

    let mut d: Vec<Vec<usize>> = vec![vec![0;t_len+1];s_len+1];
    for (i, c) in d[0].iter_mut().enumerate() {
        *c = i;
    }
    for (i, r) in d.iter_mut().enumerate() {
        r[0] = i;
    }

    for s in 1..(s_len+1) {
        for t in 1..(t_len+1) {
            let del = d[s - 1][t] + 1;
            let ins = d[s][t - 1] + 1;
            let mis = d[s - 1][t - 1] + 1;
            let mth = d[s - 1][t - 1];
            if source[s - 1] == target[t - 1] {
                d[s][t] = usize::min(mth, usize::min(ins, del));
            } else {
                d[s][t] = usize::min(mis, usize::min(ins, del));
            }
        }
    }

    count_lcs(d)
}

fn count_lcs(d: Vec<Vec<usize>>) -> usize {
    let mut lcs2_len = 0;

    let mut s = d.len() - 1;
    let mut t = d[0].len() - 1;
    while s != 0 && t != 0 {
        if s > 0 && d[s][t] == d[s - 1][t] + 1 {
            s -= 1;
        } else if t > 0 && d[s][t] == d[s][t - 1] + 1 {
            t -= 1;
        } else {
            if d[s][t] == d[s - 1][t - 1] {
                lcs2_len += 1;
            }
            s -= 1;
            t -= 1;
        }
    }
    lcs2_len
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();  // n
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let source: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap(); // m
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let target: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let lcs = lcs2(&source[..], &target[..]);
    println!("{}", lcs);
}

#[test]
fn lcs2_test() {
    let lcs = lcs2(&[2, 7, 5], &[2, 5]);
    assert_eq!(2, lcs);

    let lcs = lcs2(&[7], &[1, 2, 3, 4]);
    assert_eq!(0, lcs);

    let lcs = lcs2(&[2, 7, 8, 3], &[5, 2, 8, 7]);
    assert_eq!(2, lcs);

    let lcs = lcs2(&[1, 3, 4, 5], &[3, 4, 1, 5]);
    assert_eq!(3, lcs);
}
