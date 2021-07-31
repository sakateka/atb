use std::io;
use std::cmp;

fn find(dset: &mut Vec<(usize, u64)>, idx: usize) -> (usize, u64) {
    let (mut root, mut rows) = dset[idx as usize];
    if idx != root {
        let (r, w) = find(dset, root);
        root = r;
        rows = w;
        dset[idx as usize].0 = root;
    }
    (root, rows)
}

fn union(dset: &mut Vec<(usize, u64)>, i: usize, j: usize) -> u64 {
    let (i_idx, i_rows) = find(dset, i);
    let (j_idx, j_rows) = find(dset, j);
    let mut max = cmp::max(i_rows, j_rows);
    if i_idx == j_idx {
        return max;
    }
    max = j_rows + i_rows;
    dset[i_idx as usize] = (j_idx, 0);
    dset[j_idx as usize] = (j_idx, max);
    max
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_tables = buf.split_whitespace().nth(0).unwrap().parse::<usize>().unwrap();
    let mut array = Vec::with_capacity(num_tables);
    buf.clear();

    let mut max = 0;
    io::stdin().read_line(&mut buf).unwrap();
    for (idx, num_rows) in buf.split_whitespace().enumerate() {
        let num_rows = num_rows.parse::<u64>().unwrap();
        max = cmp::max(max, num_rows);
        array.push((idx, num_rows));
    }
    buf.clear();

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        let merge = buf[..buf.len()-1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (from, to) = (merge[0]-1, merge[1]-1); // 1 - base indexes
        max = cmp::max(max, union(&mut array, from, to));
        println!("{}", max);
        buf.clear();
    }
}

