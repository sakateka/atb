use std::io;

fn edit_distance(source: &str, target: &str) -> usize {
    let s_chars: Vec<char> = source.chars().collect();
    let t_chars: Vec<char> = target.chars().collect();
    let s_len = s_chars.len();
    let t_len = t_chars.len();

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
            if s_chars[s - 1] == t_chars[t - 1] {
                d[s][t] = usize::min(mth, usize::min(ins, del));
            } else {
                d[s][t] = usize::min(mis, usize::min(ins, del));
            }
        }
    }
    d[s_len][t_len]
}

pub fn main() {
    let mut source = String::new();
    io::stdin().read_line(&mut source).unwrap();
    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();

    let distance = edit_distance(&source, &target);
    println!("{}", distance);
}

#[test]
fn edit_distance_test() {
    let distance = edit_distance("editing", "distance");
    assert_eq!(5, distance);

    let distance = edit_distance("short", "ports");
    assert_eq!(3, distance);

    let distance = edit_distance("ab", "ab");
    assert_eq!(0, distance);
}
