use std::io;

#[derive(Debug)]
struct Node {
    idx: usize,
    parent: i32,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize) -> Self {
        Node{
            idx: idx,
            parent: -1,
            children: Vec::new(),
        }
    }
}

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf[..buf.len()-1].parse::<usize>().unwrap();
    buf.clear();

    let mut tree = (0..size).map(|idx|Node::new(idx)).collect::<Vec<Node>>();

    // populate
    // NOTE: Tree are not binary tree
    io::stdin().read_line(&mut buf).unwrap();
    for (idx, num) in buf.split_whitespace().enumerate() {
        let parent = num.parse::<i32>().unwrap();
        tree[idx].parent = parent;
        if parent > -1 {
            tree[parent as usize].children.push(idx);
        }
    }
}
