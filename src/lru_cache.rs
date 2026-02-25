#[derive(Debug)]
struct ListNode {
    val: i32,
    prev: Option<*mut ListNode>,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, prev: None, next: None }
    }
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: Option<Box<ListNode>>,
    tail: Option<*mut ListNode>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn push_back(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.prev = self.tail;

        let node_ptr: *mut _ = &mut *new_node;
        if let Some(tail) = self.tail {
            unsafe { (*tail).next = Some(new_node); }
        } else {
            self.head = Some(new_node);
        }

        self.tail = Some(node_ptr);
    }
}

pub fn main () {
    let mut dll = DoublyLinkedList::new();
    dll.push_back(1);
    dll.push_back(2);
    dll.push_back(3);
    dll.push_back(4);
    println!("{:?}", dll);
}
