use std::io;

fn left_child(idx: usize) -> usize {
    2*idx + 1
}

fn right_child(idx: usize) -> usize {
    2*idx + 2
}

fn sift_down(arr: &mut Vec<u32>, mut idx: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    loop {
        let mut max_idx = idx;
        let left = left_child(idx);
        if left < arr.len() && arr[left] < arr[max_idx] {
            max_idx = left;
        }

        let right = right_child(idx);
        if right < arr.len() && arr[right] < arr[max_idx] {
            max_idx = right;
        }
        if idx == max_idx {
            break;
        }
        pairs.push((idx, max_idx));
        let tmp = arr[idx];
        arr[idx] = arr[max_idx];
        arr[max_idx] = tmp;
        idx = max_idx;
    }
    pairs
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf[..buf.len()-1].parse::<usize>().unwrap();
    let mut array = Vec::with_capacity(size);
    buf.clear();

    io::stdin().read_line(&mut buf).unwrap();
    for num in buf.split_whitespace() {
        let num = num.parse::<u32>().unwrap();
        array.push(num);
    }
    let size = array.len();
    let mut pairs = Vec::new();
    for i in (0..(size/2 + 1)).rev() {
        pairs.extend(sift_down(&mut array, i).iter().cloned());
    }
    println!("{}", pairs.len());
    for (i1, i2) in pairs {
        println!("{} {}", i1, i2);
    }
}
