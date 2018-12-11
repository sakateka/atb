use std::io;

fn knapsack(weights: &[usize], bars: &[usize]) -> usize {
    let w_len = weights.len();
    let i_len = bars.len();

    let mut d: Vec<Vec<usize>> = vec![vec![0;i_len+1];w_len+1];
    for i in 1..(i_len+1) {
        for w in 1..(w_len+1) {
            d[w][i] = d[w][i - 1];
            let bar_w = bars[i - 1];
            if bar_w <= w {
                let val = d[w - bar_w][i - 1] + bar_w;
                if d[w][i] < val {
                    d[w][i] = val;
                }
            }
        }
    }
    d[w_len][i_len]
}

pub fn main() {
    let mut wn = String::new();
    io::stdin().read_line(&mut wn).unwrap();
    let big_w: usize = wn.split_whitespace()
        .map(|x| x.parse().unwrap())
        .take(1)
        .next()
        .unwrap();

    let weights: Vec<usize> = (1..(big_w+1)).collect();
    let mut bars = String::new();
    io::stdin().read_line(&mut bars).unwrap();
    let bars: Vec<usize> = bars.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let weight = knapsack(&weights[..], &bars[..]);
    println!("{}", weight);
}

#[test]
fn knapsack_test() {
    let weights: Vec<usize> = (1..11).collect(); // W=10
    let weight = knapsack(&weights[..], &[1, 4, 8]);
    assert_eq!(9, weight);

    let weights: Vec<usize> = (1..48).collect(); // W=47
    let weight = knapsack(&weights[..], &[9, 14, 16, 30]);
    assert_eq!(46, weight);
}
