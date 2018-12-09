use std::io;

fn primitive_calculator(num: u32) -> Vec<usize> {
    let num = num as usize;
    let mut ops = vec![0;num+1];
    let mut path = vec![1_usize];

    for m in 2..(num+1) {
        ops[m] = u32::max_value();
        for n in (1..4).map(|x| match x {
            1 => m - 1,
            2 => if m % 2 == 0 { m / 2 } else { m },
            3 => if m % 3 == 0 { m / 3 } else { m },
            _ => unreachable!(),
        }) {
            if m > n {
                let num_ops = ops[n] + 1;
                if num_ops < ops[m] {
                    ops[m] = num_ops;
                }
            }
        }
        path.push(ops[m] as usize);
    }
    let mut path2 = vec![1];
    let mut s = num;
    for (i, &g) in path.iter().rev().enumerate() {
        if g < s {
            path2.insert(1, num - i);
            s = g;
        }
    }
    path2
}

fn primitive_calculator_direct_path(num: u32) -> Vec<usize> {
    let num = num as usize;
    let mut ops = vec![0;num+1];
    let mut path = vec![1_usize];

    for m in 2..(num+1) {
        ops[m] = u32::max_value();
        for n in (1..4).map(|x| match x {
            1 => m - 1,
            2 => if m % 2 == 0 { m / 2 } else { m },
            3 => if m % 3 == 0 { m / 3 } else { m },
            _ => unreachable!(),
        }) {
            if m > n {
                let num_ops = ops[n] + 1;
                if num_ops < ops[m] {
                    ops[m] = num_ops;
                }
            }
        }
        path.push(ops[m] as usize);
    }
    let mut path2 = vec![1];
    let mut s = num;
    for (i, &g) in path.iter().rev().enumerate() {
        if g < s {
            path2.insert(1, num - i);
            s = g;
        }
    }
    path2
}


pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: u32 = buf.trim().parse().unwrap();
    let path = primitive_calculator(num);
    println!("{}", path.len() - 1);
    for p in path {
        print!("{} ", p);
    }
    println!();
}

#[test]
fn primitive_calculator_test() {
    assert_eq!(vec![1], primitive_calculator(1));

    let res = primitive_calculator(5);
    assert!(vec![1, 3, 4, 5] == res || vec![1, 2, 4, 5] == res);

    let res = primitive_calculator(96234);
    let expect_v1 = vec![
        1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 78732, 91854, 96228, 96234];
    let expect_v2 = vec![
        1, 3, 9, 10, 11, 22,   66,  198,  594,  1782,  5346, 16038, 16039, 32078, 96234];
    assert!(expect_v1 == res || expect_v2 == res);
}
