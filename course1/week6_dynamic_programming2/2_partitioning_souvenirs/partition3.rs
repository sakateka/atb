use std::io;

fn partition3(bars: &[usize]) -> usize {
    let sum = bars.iter().fold(0, |acc, x| acc + x);
    if sum % 3 != 0 {
        return 0;
    }
    let big_w = sum / 3;

    let i_len = bars.len();

    let mut d: Vec<Vec<usize>> = vec![vec![0;big_w+1];i_len+1];
    for w in 1..(big_w+1) {
        for i in 1..(i_len+1) {
            d[i][w] = d[i - 1][w];
            let bar_w = bars[i - 1];
            if bar_w == w || (bar_w < w && d[i - 1][w - bar_w] > 0) {
                d[i][w] = if d[i-1][w] == 0 {1} else {2};
            }
        }
    }
    if d[i_len][big_w] == 2 {1} else {0}
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();

    io::stdin().read_line(&mut buf).unwrap();
    let bars: Vec<usize> = buf.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let fair = partition3(&bars[..]);
    println!("{}", fair);
}

#[test]
fn knapsack_test() {
    assert_eq!(0, partition3(&[40]));
    assert_eq!(0, partition3(&[3, 3, 3, 3]));
    assert_eq!(0, partition3(&[1, 1, 2, 2, 2, 2, 3, 3]));
    assert_eq!(1, partition3(&[1, 1, 2, 2, 3, 3]));
    assert_eq!(1, partition3(&[7, 5, 2, 3, 3, 1]));
    assert_eq!(1, partition3(&[3, 3, 3]));
    assert_eq!(1, partition3(&[17, 59, 34, 57, 17, 23, 67, 1, 18, 2, 59]));
    assert_eq!(1, partition3(&[1, 2, 3, 4, 5, 5, 7, 7, 8, 10, 12, 19, 25]));
}
