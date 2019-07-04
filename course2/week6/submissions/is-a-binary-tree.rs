use std::io;

#[derive(Debug)]
struct Node {
    key: i64,
    left: i64,
    right: i64,
}

impl Node {
    fn new(data: &[i64]) -> Self {
        Node {
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
        Tree(Vec::with_capacity(cap))
    }
    fn push(&mut self, n: Node) {
        self.0.push(n);
    }

    fn is_bst(&self, idx: i64) -> bool {
        /*
        bool is_search_tree = true;
        if (node != NULL && (
               (node->left != NULL && (
                       node->data <= node->left->data || ! (
                           node->left->right == NULL || node->data > node->left->right->data
                       ) || !tree_is_search_tree(node->left)
                   )
               )||(node->right != NULL && (
                       node->data >= node->right->data || ! (
                           node->right->left == NULL || node->data < node->right->left->data
                       ) || !tree_is_search_tree(node->right)
                   )
               )
           )
        ) {
            is_search_tree = false;
        }
        return is_search_tree;
        */
        if self.0.is_empty() {
            return true;
        }
        let node = &self.0[idx as usize];

        if node.left != -1
            && (node.key <= self.0[node.left as usize].key
                || (self.0[node.left as usize].right != -1
                    && node.key <= self.0[self.0[node.left as usize].right as usize].key)
                || !self.is_bst(node.left))
            || (node.right != -1
                && (node.key > self.0[node.right as usize].key
                    || (self.0[node.right as usize].left != -1
                        && node.key > self.0[self.0[node.right as usize].left as usize].key)
                    || !self.is_bst(node.right)))
        {
            return false;
        }
        return true;
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

fn run() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf[..buf.len() - 1].parse::<usize>().unwrap();
    buf.clear();

    let mut tree = Tree::new(size);

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        let node = buf
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        tree.push(Node::new(&node[..3]));
        buf.clear();
    }
    if tree.is_bst(0) {
        println!("CORRECT");
    } else {
        println!("INCORRECT");
    }
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
