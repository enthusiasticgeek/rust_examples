struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }
    
    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.data.clear();
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("Queue size: {}", queue.size());
    println!("Queue is empty: {}", queue.is_empty());


    while let Some(item) = queue.dequeue() {
        println!("Dequeued item: {}", item);
    }

    println!("Queue is empty: {}", queue.is_empty());
}

