struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.stack.len()
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(item) = stack.pop() {
        println!("Popped item: {}", item);
    }
}

