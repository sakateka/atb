use std::io::{self, Read};
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

const START: char =  'l';
const POINT: char =  'p';
const END: char =  'r';

fn fast_count_segments<'a>(dots: &Vec<(&'a i32, char)>, points: &[i32]) -> Vec<u32> {
    let mut count = vec![0; points.len()];
    let mut idx :HashMap<&i32, Vec<usize>> = HashMap::new();
    let mut n = 0;
    for p in points.iter().enumerate() {
        let i = idx.entry(p.1).or_insert(Vec::new());
        i.push(p.0);
    }

    for d in dots {
        match d.1 {
            START => n += 1,
            POINT => {
                for point_idx in idx[d.0].iter() {
                    count[*point_idx] = n;
                }
            }
            END => n -= 1,
            _ => unreachable!(),
        }
    }
    count
}

fn build_dots(numbers: &[i32], dots_start_at: usize) -> Vec<(&i32, char)> {
    let mut dots: Vec<(&i32, char)> = numbers.iter().enumerate().map(|i| {
        let mut atype = POINT;
        if i.0 < dots_start_at {
            if i.0 % 2 == 0 {
                atype = START;
            } else {
                atype = END;
            }
        }
        (i.1, atype)
    }).collect();
    dots.sort_by(|a, b| {
        let cmp_dot = (*a.0).cmp(b.0);
        if cmp_dot == Equal {
            a.1.cmp(&b.1)
        } else {
            cmp_dot
        }
    });
    dots
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let numbers: Vec<i32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    let n = numbers[0] as usize;
    let s_dots = n * 2;

    let dots = build_dots(&numbers[2..], s_dots);
    let count = fast_count_segments(&dots, &numbers[2+n*2..]);
    println!("{}", count.iter().fold(String::new(), |acc, num|{
        let sep = if acc.len() > 0 { " " } else { "" };
        acc + sep + &num.to_string()
    }));
}


#[test]
fn fast_count_segments_test() {
    let cases: Vec<(Vec<i32>, Vec<u32>)> = vec![
        /* n, m, ss1, se1, ss2, se2, p1, p2 */               /* counts */
        (vec![4, 4, 0, 1, 0, 2, 0, 3, 0, 4, 1, 2, 3, 4], vec![4, 3, 2, 1]),
        (vec![4, 4, 1, 1, 2, 2, 3, 3, 4, 4, 1, 2, 3, 4], vec![1, 1, 1, 1]),
        (vec![4, 4, 1, 1, 2, 2, 3, 3, 4, 4, 0, 1, 2, 5], vec![0, 1, 1, 0]),
        (vec![4, 4, 0, 2, 1, 3, 2, 4, 3, 5, 1, 2, 3, 4], vec![2, 3, 3, 2]),
        (vec![4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1], vec![4, 4, 0, 0]),
        (vec![4, 4, 0, 3, 1, 3, 2, 3, 3, 3, 0, 1, 2, 3], vec![1, 2, 3, 4]),
        (vec![4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![4]),
        (vec![1, 6, 0, 4, -1, 0, 0, 3, 4, 5], vec![0, 1, 1, 1, 1, 0]),
        (vec![5, 2, -100, -100, -10, -10, 0, 0, 10, 10, 100, 100, -100, -10], vec![1, 1]),
    ];
    for c in cases {
        let n = c.0[0] as usize;
        let s_dots = n*2;
        let mut dots = build_dots(&c.0[2..], s_dots);
        let count = fast_count_segments(&mut dots, &c.0[2+n*2..]);
        println!("case {:?}, counts: {:?}", c, count);
        let expect = c.1;
        assert_eq!(expect, count);
    }
}
