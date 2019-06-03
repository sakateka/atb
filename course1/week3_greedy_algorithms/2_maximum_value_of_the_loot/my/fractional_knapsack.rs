use std::io::{self, Read};
use std::cmp::Ordering::Equal;

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn get_optimal_value(mut capacity: f64, mut items: Vec<Item>) -> f64 {
    let mut v = 0.;
    let mut index: Vec<usize> = (0..items.len()).collect::<Vec<usize>>();
    index.sort_by(|a, b|{
        let cost_a = items[*a].value/items[*a].weight;
        let cost_b = items[*b].value/items[*b].weight;
        // reverse order
        cost_b.partial_cmp(&cost_a).unwrap_or(Equal)
    });
    for i in index {
        if capacity == 0.00000 {
            break;
        }
        let add_w = min(items[i].weight, capacity);
        v += add_w * (items[i].value / items[i].weight);
        items[i].weight -= add_w;
        capacity -= add_w;
    }
    v
}

struct Item {
    weight: f64,
    value: f64,
}


pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let numbers: Vec<f64> = buf.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

    let n = numbers[0] as u32;
    let capacity = numbers[1];

    let items: Vec<Item> = (0..n).map(|i| {
        Item{
            value: numbers[(i*2 + 2) as usize],
            weight: numbers[(i*2 + 3) as usize],
        }
    }).collect();

    println!("{:0.3}", get_optimal_value(capacity, items));
}
