use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::cmp::PartialOrd;
use std::clone::Clone;

struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

impl<T: fmt::Display> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct BinaryTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: fmt::Display + PartialOrd + Clone> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: T) {
        if let Some(root) = self.root.clone() {
            self.insert_recursive(value, root);
        } else {
            self.root = Some(Node::new(value));
        }
    }

    fn insert_recursive(&mut self, value: T, node: Rc<RefCell<Node<T>>>) {
        let mut borrowed_node = node.borrow_mut();
        if value < borrowed_node.value {
            if let Some(left) = borrowed_node.left.clone() {
                self.insert_recursive(value, left);
            } else {
                borrowed_node.left = Some(Node::new(value));
            }
        } else {
            if let Some(right) = borrowed_node.right.clone() {
                self.insert_recursive(value, right);
            } else {
                borrowed_node.right = Some(Node::new(value));
            }
        }
    }

    fn display_inorder(&self) {
        self.inorder_recursive(self.root.as_ref().map(Rc::clone));
        println!();
    }

    fn inorder_recursive(&self, node: Option<Rc<RefCell<Node<T>>>>) {
        if let Some(node) = node {
            let borrowed_node = node.borrow();
            self.inorder_recursive(borrowed_node.left.as_ref().map(Rc::clone));
            print!("{} ", borrowed_node.value);
            self.inorder_recursive(borrowed_node.right.as_ref().map(Rc::clone));
        }
    }

    fn display_visual(&self) {
        if let Some(root) = &self.root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(Rc::clone(root));

            while !queue.is_empty() {
                let level_size = queue.len();
                for _ in 0..level_size {
                    let node = queue.pop_front().unwrap();
                    let borrowed_node = node.borrow();
                    print!("{} [ ", borrowed_node.value);
                    if let Some(left) = borrowed_node.left.as_ref() {
                        let borrowed_left = left.borrow();
                        print!("L: {} ", borrowed_left.value);
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = borrowed_node.right.as_ref() {
                        let borrowed_right = right.borrow();
                        print!("R: {} ", borrowed_right.value);
                        queue.push_back(Rc::clone(right));
                    }
                    println!("]");
                }
                println!();
            }
        }
    }

    //breadth first search (bfs)
    fn bfs(&self) {
        if let Some(root) = &self.root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(Rc::clone(root));

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                let borrowed_node = node.borrow();
                print!("{} ", borrowed_node.value);

                if let Some(left) = borrowed_node.left.as_ref() {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(right) = borrowed_node.right.as_ref() {
                    queue.push_back(Rc::clone(right));
                }
            }
        }
        println!();
    }

    //depth first search (dfs)
    fn dfs(&self) {
        self.dfs_recursive(self.root.as_ref().map(Rc::clone));
        println!();
    }

    fn dfs_recursive(&self, node: Option<Rc<RefCell<Node<T>>>>) {
        if let Some(node) = node {
            let borrowed_node = node.borrow();
            print!("{} ", borrowed_node.value);
            self.dfs_recursive(borrowed_node.left.as_ref().map(Rc::clone));
            self.dfs_recursive(borrowed_node.right.as_ref().map(Rc::clone));
        }
    }
}

fn main() {
    let mut tree: BinaryTree<i32> = BinaryTree::new();

    tree.insert(4);
    tree.insert(2);
    tree.insert(6);
    tree.insert(1);
    tree.insert(3);
    tree.insert(5);
    tree.insert(7);

    tree.display_inorder();
    println!("Visual Display:");
    tree.display_visual();

    println!("Removing node 4");
    //tree.remove(4);

    tree.display_inorder();
    println!("Visual Display:");
    tree.display_visual();
    
    println!("Breadth First Search (BFS):");
    tree.bfs();
    println!("Depth First Search (DFS):");
    tree.dfs();
}


