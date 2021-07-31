use std::io;

fn left_child(idx: usize) -> usize {
    2*idx + 1
}

fn right_child(idx: usize) -> usize {
    2*idx + 2
}

fn swap(arr: &mut Vec<(u64, u32)>, from: usize, to: usize) {
    let tmp = arr[from];
    arr[from] = arr[to];
    arr[to] = tmp;
}

fn sift_down(arr: &mut Vec<(u64, u32)>, mut idx: usize) {
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
        swap(arr, idx, max_idx);
        idx = max_idx;
    }
}
/*
fn parent(idx: usize) -> usize {
    if idx == 0 {
        0
    } else {
        (idx - 1) / 2
    }
}

fn sift_up(arr: &mut Vec<(u64, u32)>, mut idx: usize) {
    let mut p_idx = parent(idx);
    while idx > 0 && arr[p_idx] > arr[idx] {
        swap(arr, p_idx, idx);
        idx = p_idx;
        p_idx = parent(idx);
    }
}
*/

fn schedule_task(arr: &mut Vec<(u64, u32)>, duration: u64) -> (u32, u64) {
    let (time, cpu) = arr[0];
    arr[0] = (time+duration, cpu);
    sift_down(arr, 0);
    (cpu, time)
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_cpus = buf.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap();
    let mut array = Vec::with_capacity(num_cpus as usize);
    for cpu in 0..num_cpus {
        array.push((0u64, cpu));
    }
    buf.clear();

    io::stdin().read_line(&mut buf).unwrap();
    for duration in buf.split_whitespace() {
        let duration = duration.parse::<u64>().unwrap();
        let (cpu, time) = schedule_task(&mut array, duration);
        println!("{} {}", cpu, time);
    }
}

#[test]
fn test1() {
    //2 5
    //1 2 3 4 5
    let mut workers = vec![(0, 0), (0, 1)];
    let mut result = vec![];
    let expect = vec![(0,0), (1,0), (0,1), (1,2), (0,4)];

    for dur in [1u64, 2, 3, 4, 5].iter() {
        result.push(schedule_task(&mut workers, *dur));
    }

    assert_eq!(result, expect);
}

#[test]
fn test2() {
    //4 20
    //1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
    let mut workers = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let mut result = vec![];
    let expect = vec![(0,0), (1,0), (2,0), (3,0), (0,1), (1,1), (2,1),
                      (3,1), (0,2), (1,2), (2,2), (3,2), (0,3), (1,3),
                      (2,3), (3,3), (0,4), (1,4), (2,4), (3,4)];

    for dur in [1; 20].iter() {
        result.push(schedule_task(&mut workers, *dur));
    }

    assert_eq!(result, expect);
}
