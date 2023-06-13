//This is a demonstration of a simple linked list data structure
use std::fmt;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
  
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {     
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
  
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
  
    fn display(&self)
        where
            T: fmt::Debug,
        {
            let mut current = &self.head;
            while let Some(node) = current {
                print!("{:?} -> ", node.data);
                current = &node.next;
            }
            println!("None");
        }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{:?} -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(3);
    list.push(2);
    list.push(1);
    list.push(2);
    list.push(3);
    list.display();
    println!("Popped element: {:?}", list.pop());
    println!("Debug output: {:?}", list);
}

