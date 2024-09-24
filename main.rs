use std::cmp::Ordering;

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Node {
    value: i32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left_child: None,
            right_child: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            },
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        match value.cmp(&node.value) {
            Ordering::Greater => {
                match &mut node.right_child {
                    None => {
                        node.right_child = Node::new(value).into();
                    },
                    Some(node) => {
                        Tree::insert_recursive(node, value);
                    }
                }
            },
            Ordering::Less => {
                match &mut node.left_child {
                    None => {
                        node.left_child = Node::new(value).into();
                    },
                    Some(node) => {
                        Tree::insert_recursive(node, value);
                    }
                }
            },
            Ordering::Equal => {
                panic!("Binary Tree cannot have duplicate values");
            }
        }
    }

    fn remove(&mut self, value: i32) {
        self.root = match &mut self.root {
            None => None,
            Some(node) => {
                Tree::remove_recursive(node, value)
            }
        };
    }

    fn remove_recursive(node: &mut Box<Node>, value: i32) -> Option<Box<Node>> {
        match value.cmp(&node.value) {
            Ordering::Greater => {
                match &mut node.right_child {
                    None => None,
                    Some(child) => {
                        node.right_child = Tree::remove_recursive(child, value);
                        Some(node.clone())
                    }
                }
            },
            Ordering::Less => {
                match &mut node.left_child {
                    None => None,
                    Some(child) => {
                        node.left_child = Tree::remove_recursive(child, value);
                        Some(node.clone())
                    }
                }
            },
            Ordering::Equal => {
                if node.left_child.is_none() && node.right_child.is_none() {
                    return None;
                }

                if node.left_child.is_none() {
                    return node.right_child.take();
                }

                if node.right_child.is_none() {
                    return node.left_child.take();
                }

                let mut successor = node.right_child.as_mut().unwrap();
                while successor.left_child.is_some() {
                    successor = successor.left_child.as_mut().unwrap();
                }

                node.value = successor.value;

                node.right_child = Tree::remove_recursive(successor, successor.value);

                Some(node.clone())
            }
        }
    }

    fn print_tree(&self) {
        self.print_tree_recursive(&self.root, 0);
    }

    fn print_tree_recursive(&self, node: &Option<Box<Node>>, level: usize) {
        if let Some(node) = node {
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", node.value);
            self.print_tree_recursive(&node.right_child, level + 1);
            self.print_tree_recursive(&node.left_child, level + 1);
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(2);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    tree.print_tree();

    tree.remove(3);

    tree.print_tree();
}

