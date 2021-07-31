use std::io;
use std::cmp;

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

fn tree_height(tree: &Vec<Node>, from: usize) -> usize {
    let node = &tree[from];
    let mut max_size = 0;
    for c in node.children.iter() {
        max_size = cmp::max(max_size, tree_height(tree, tree[*c].idx));
    }
    max_size + 1
}

fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf[..buf.len()-1].parse::<usize>().unwrap();
    buf.clear();

    let mut root: usize = 0;
    let mut tree = (0..size).map(|idx|Node::new(idx)).collect::<Vec<Node>>();

    // populate
    // NOTE: Tree are not binary tree
    io::stdin().read_line(&mut buf).unwrap();
    for (idx, num) in buf.split_whitespace().enumerate() {
        let parent = num.parse::<i32>().unwrap();
        tree[idx].parent = parent;
        if parent > -1 {
            tree[parent as usize].children.push(idx);
        } else {
            root = idx;
        }
    }

    let height = tree_height(&tree, root);
    println!("{}", height);
}

use std::thread;
const STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
