use std::io::{self, Read};

const MIN_DISTANCE: f64 = 1_000_000_000_000_000_000_f64;

fn naive_minimum_distance(points: &mut [&[i32]], limit: Option<usize>) -> f64 {
    let mut min_distance: f64 = MIN_DISTANCE;
    let is_limit = limit.is_some();
    let limit = limit.unwrap_or(0);
    for (i, p1) in points.iter().enumerate() {
        let mut sub_list = &points[(i+1)..];
        if is_limit {
            sub_list = &sub_list[..usize::min(sub_list.len(), i+limit)];
        }

        for p2 in sub_list {
            let distance = (
                (i32::pow((p1[0] - p2[0]), 2) + i32::pow((p1[1] - p2[1]), 2))
            as f64).sqrt();
            if distance < min_distance {
                min_distance = distance;
            }
        }

    }
    min_distance
}

fn minimum_distance(points: &mut [&[i32]]) -> f64 {
    let points_len = points.len();
    if points_len <= 3 {
        return naive_minimum_distance(points, None);
    }

    let center_idx = points_len/2;
    let left_min = minimum_distance(&mut points[..center_idx]);
    let right_min = minimum_distance(&mut points[center_idx..]);

    let center = points[center_idx][0];

    let mut left_idx = center_idx - 1;
    while left_idx > 0 && ((center - points[left_idx][0]) as f64) < left_min {
        left_idx -= 1;
    }
    let mut right_idx = center_idx + 1;
    while right_idx < points_len && ((points[right_idx][0] - center) as f64) < right_min {
        right_idx += 1;
    }

    let mut center_points: Vec<&[i32]> = points[left_idx..right_idx].iter().cloned().collect();
    center_points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
    let center_min = naive_minimum_distance(&mut center_points[..], Some(7));
    return f64::min(left_min, f64::min(right_min, center_min))
}

fn make_points(numbers: &mut [i32]) -> Vec<&[i32]> {
    let mut points = numbers.chunks(2).collect::<Vec<&[i32]>>();
    points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    points
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut numbers: Vec<i32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    let _n = numbers[0] as usize;
    let mut points = make_points(&mut numbers[1..]);
    println!("{}", minimum_distance(&mut points[..]));
}


#[test]
fn minimum_distance_test() {
    let cases: Vec<Vec<i32>> = vec![
        vec![0, 0, 3, 4],
        vec![7, 7, 1, 100, 4, 8, 7, 7],
        vec![4, 4, -2, -2, -3, -4, -1, 3, 2, 3, -4, 0, 1, 1, -1, -1, 3, -1, -4, 2, -2, 4],
    ];
    for c in cases {
        let mut nc = c.clone();
        let mut fc = c.clone();
        let mut np = make_points(&mut nc[..]);
        let nd = naive_minimum_distance(&mut np[..], None);
        let mut fp = make_points(&mut fc[..]);
        let fd = minimum_distance(&mut fp[..]);
        println!("nd={}, fd={}, c={:?}", nd, fd, c);
        assert_eq!(nd, fd);
    }
}
