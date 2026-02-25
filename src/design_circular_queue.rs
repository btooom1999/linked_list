#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct MyCircularQueue {
    queue: Option<Box<ListNode>>,
    size: i32,
    capacity: i32,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self { queue: None, size: 0, capacity: k }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.size >= self.capacity {
            return false;
        }

        let mut head = &mut self.queue;
        while let Some(node) = head {
            head = &mut node.next;
        }

        *head = Some(Box::new(ListNode::new(value)));
        self.size += 1;

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.size <= 0 {
            return false;
        }

        self.queue = self.queue.as_mut().unwrap().next.take();
        self.size -= 1;

        true
    }

    fn front(&self) -> i32 {
        if let Some(node) = self.queue.as_ref() {
            node.val
        } else {
            -1
        }
    }

    fn rear(&self) -> i32 {
        let mut head = &self.queue;
        while let Some(node) = head && node.as_ref().next.is_some() {
            head = &node.next;
        }

        if let Some(node) = head {
            node.val
        } else {
            -1
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

pub fn main() {
    let mut my_circular_queue = MyCircularQueue::new(3);
    println!("{}", my_circular_queue.en_queue(1));
    println!("{}", my_circular_queue.en_queue(2));
    println!("{}", my_circular_queue.en_queue(3));
    println!("{}", my_circular_queue.en_queue(4));
    println!("{}", my_circular_queue.rear());
    println!("{}", my_circular_queue.is_full());
    println!("{}", my_circular_queue.de_queue());
    println!("{}", my_circular_queue.en_queue(4));
    println!("{}", my_circular_queue.rear());
}
