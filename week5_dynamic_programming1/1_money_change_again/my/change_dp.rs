use std::io;

pub fn change_dp(money: u32, coins: &[u32]) -> u32 {
    let money = money as usize;
    let mut min_num_coins = vec![0;money+1];
    for m in 1..(money+1) {
        min_num_coins[m] = u32::max_value();
        for c in coins.iter() {
            let c = *c as usize;
            if m >= c {
                let num_coins = min_num_coins[m - c] + 1;
                if num_coins < min_num_coins[m] {
                    min_num_coins[m] = num_coins;
                }
            }
        }
    }
    min_num_coins[money]
}

pub fn recursive_change(money: u32, coins: &[u32]) -> u32 {
    if money == 0 {
        return 0;
    }

    let mut min_num_coins = u32::max_value();
    for coin in coins.iter() {
        if &money >= coin {
            let num_coins = recursive_change(money - coin, coins) + 1; 
            if num_coins < min_num_coins {
                min_num_coins = num_coins;
            }
        }
    }
    min_num_coins
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let money: u32 = buf.trim().parse().unwrap();
    let coins = [1, 3, 4];
    println!("{}", change_dp(money, &coins));
}

#[test]
fn recursive_change_test() {
    let coins = [1, 3, 4];
    assert_eq!(2, recursive_change(2, &coins));
    assert_eq!(9, recursive_change(34, &coins));
}

#[test]
fn change_dp_test() {
    use std::time::Instant;

    let coins = [1, 3, 4];
    let cases = [2, 34, 37];//, 72];
    for money in cases.iter().cloned() {
        let now = Instant::now();
        let rc = recursive_change(money, &coins);
        let end = now.elapsed();
        let rec_time  = end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9;
        let now = Instant::now();
        let dpc = change_dp(money, &coins);
        let end = now.elapsed();
        let dp_time  = end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9;
        println!("rec_time={:0.6}, dp_time={:0.6}", rec_time, dp_time);
        assert_eq!(rc, dpc);
    }
}
