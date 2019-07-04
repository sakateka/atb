use std::io;

#[derive(Debug)]
struct Node {
    key: i64,
    left: i64,
    right: i64,
}

impl Node {
    fn new(data: &[i64]) -> Self {
        Node{
            key: data[0],
            left: data[1],
            right: data[2],
        }
    }
}

#[derive(Debug)]
struct Tree(Vec<Node>);

impl Tree {
    fn new(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }
    fn push(&mut self, n: Node) {
        self.0.push(n);
    }

    fn in_order_traversal(&self, idx: i64) {
        if idx == -1 {
            return;
        }
        let node = &self.0[idx as usize];
        self.in_order_traversal(node.left);
        print!("{} ", node.key);
        self.in_order_traversal(node.right);
    }

    fn pre_order_traversal(&self, idx: i64) {
        if idx == -1 {
            return;
        }
        let node = &self.0[idx as usize];
        print!("{} ", node.key);
        self.pre_order_traversal(node.left);
        self.pre_order_traversal(node.right);
    }

    fn post_order_traversal(&self, idx: i64) {
        if idx == -1 {
            return;
        }
        let node = &self.0[idx as usize];
        self.post_order_traversal(node.left);
        self.post_order_traversal(node.right);
        print!("{} ", node.key);
    }
}


fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf[..buf.len()-1].parse::<usize>().unwrap();
    buf.clear();

    let mut tree = Tree::new(size);

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        let node = buf.split_whitespace()
            .map(|i|i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        tree.push(Node::new(&node[..3]));
        buf.clear();
    }
    tree.in_order_traversal(0);
    println!();
    tree.pre_order_traversal(0);
    println!();
    tree.post_order_traversal(0);
    println!();
}
