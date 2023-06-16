use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Debug;

struct Node<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

struct DoubleLinkedList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug> DoubleLinkedList<T> {
    fn new() -> DoubleLinkedList<T> {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        self.head = Some(new_node);
    }

    fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                let new_tail = Rc::downgrade(&new_node);
                old_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().and_then(|old_tail| {
            old_tail.upgrade().and_then(|tail| {
                match tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.upgrade().unwrap().borrow_mut().next = None;
                        self.tail = Some(new_tail);
                    }
                    None => {
                        self.head.take();
                    }
                }
                //Rc::try_unwrap(tail).ok().unwrap().into_inner().value
                Some(Rc::try_unwrap(tail).ok().unwrap().into_inner().value)
            })
        })
    }
}
    
fn display_list<T: Debug>(list: &DoubleLinkedList<T>) {
     let mut current = list.head.as_ref().map(|node| Rc::clone(node));
     while let Some(node) = current {
        //println!("hello");
        println!("{:?}", node.borrow().value);
        current = node.borrow().next.as_ref().map(|next| Rc::clone(next));
     }
}
    

fn main() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    list.push_back(4);
    list.pop_back();
    list.push_back(5);
    
    display_list(&list);

    while let Some(value) = list.pop_front() {
        println!("Popped front: {}", value);
    }
    // Output:
    // Popped front: 1
    
}
