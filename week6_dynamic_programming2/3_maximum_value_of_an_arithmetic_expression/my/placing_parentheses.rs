use std::io;

fn eval(a: i64, op: char, b: i64) -> i64 {
    match op {
        '-' => a - b,
        '+' => a + b,
        '*' => a * b,
        _ => unreachable!(),
    }
}

fn min_and_max(i: usize, j: usize, m: &Vec<Vec<i64>>, ops: &Vec<char>) -> (i64, i64) {
    let mut min = i64::max_value();
    let mut max = i64::min_value();
    for k in i..j {
        let a = eval(m[k][i], ops[k-1], m[j][k+1]);
        let b = eval(m[k][i], ops[k-1], m[k+1][j]);
        let c = eval(m[i][k], ops[k-1], m[j][k+1]);
        let d = eval(m[i][k], ops[k-1], m[k+1][j]);
        min = i64::min(min, i64::min(i64::min(a, b), i64::min(c, d)));
        max = i64::max(max, i64::max(i64::max(a, b), i64::max(c, d)));
    }
    (min, max)
}

fn parentheses(d: Vec<i64>, ops: Vec<char>) -> i64 {
    let n = d.len();
    let mut m = vec![vec![0;n+1];n+1];
    for i in 1..(n+1) {
        m[i][i] = d[i-1];
    }

    for s in 1..n {
        for i in 1..(n-s + 1) {
            let j = i + s;
            let (min, max) = min_and_max(i, j, &mut m, &ops);
            m[i][j] = min;
            m[j][i] = max;
        }
    }
    return m[n][1];
}

fn get_digits_and_ops(expression: &str) -> (Vec<i64>, Vec<char>) {
    let symbols: Vec<char> = expression.trim().chars().collect();
    let digits: Vec<i64> = symbols.iter()
        .enumerate()
        .filter(|x| x.0 % 2 == 0)
        .map(|x| *x.1 as i64 - 48)
        .collect();
    let ops: Vec<char> = symbols.iter().enumerate()
        .filter(|x| x.0 % 2 == 1)
        .map(|x| *x.1)
        .collect();
    (digits, ops)
}

pub fn main() {
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).unwrap();
    let (digits, ops) = get_digits_and_ops(&expression);

    println!("{}", parentheses(digits, ops));
}


#[test]
fn get_digits_and_ops_test() {
    let (digits, ops) = get_digits_and_ops("5-8+7*4-8+9");
    let expect_digits = vec![5, 8, 7, 4, 8, 9];
    let expect_ops = vec!['-', '+', '*', '-', '+'];
    assert_eq!(expect_digits, digits);
    assert_eq!(expect_ops, ops);
}

#[test]
fn parentheses_test() {
    let (digits, ops) = get_digits_and_ops("5-8+7*4-8+9");
    let max = parentheses(digits, ops);
    assert_eq!(200, max);

    let (digits, ops) = get_digits_and_ops("1+5");
    let max = parentheses(digits, ops);
    assert_eq!(6, max);

    let (digits, ops) = get_digits_and_ops("8-5*3");
    let max = parentheses(digits, ops);
    assert_eq!(9, max);
}
