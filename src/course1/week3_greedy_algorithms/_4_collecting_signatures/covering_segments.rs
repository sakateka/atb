use std::io::{self, Read};
use std::cmp::Ordering::Equal;

struct Segment {
    start: u32,
    end: u32,
}

fn optimal_points(mut segments: Vec<Segment>) -> Vec<u32> {
    let mut points = Vec::new();
    segments.sort_by(|a, b| {
        let cmp_starts = a.start.cmp(&b.start);
        if cmp_starts == Equal {
            a.end.cmp(&b.end)
        } else {
            cmp_starts
        }
    });
    points.push(segments[0].end);
    for s in segments {
        let last = *points.last().unwrap();
        if last >= s.start && last <= s.end {
            continue
        }
        if s.end < last {
            let len = points.len();
            points[len -1] = s.end;
        } else {
            points.push(s.end);
        }
    }
    points
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let numbers: Vec<u32> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    let n = numbers[0] as usize;

    let segments: Vec<Segment> = (0..n).map(|i| {
        Segment{
            start: numbers[i*2 + 1],
            end: numbers[i*2 + 2],
        }
    }).collect();

    let points = optimal_points(segments);
    println!("{}", points.len());
    println!("{}", points.iter().fold(String::new(), |acc, num|{
        let sep = if acc.len() > 0 { " " } else { "" };
        acc + sep + &num.to_string()
    }));
}
