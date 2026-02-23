#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug, Clone)]
struct MyLinkedList {
    head: Option<Box<ListNode>>,
    size: i32,
}

impl MyLinkedList {
    fn new() -> Self {
        Self { head: None, size: 0 }
    }

    fn get(&self, mut index: i32) -> i32 {
        if index < 0 || index >= self.size {
            return -1;
        }

        let mut head = self.head.as_deref();
        for _ in 0..index {
            head = head.unwrap().next.as_deref();
        }

        head.unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        let mut node = Box::new(ListNode::new(val));
        node.next = self.head.take();

        self.head = Some(node);
        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut head = &mut self.head;
        while let Some(node) = head {
            head = &mut node.next;
        }

        *head = Some(Box::new(ListNode::new(val)));
        self.size += 1;
    }

    fn add_at_index(&mut self, mut index: i32, val: i32) {
        if index < 0 || index > self.size {
            return;
        }

        let mut head = &mut self.head;
        for _ in 0..index {
            head = &mut head.as_mut().unwrap().next;
        }

        let next_node = head.take();
        let mut node = Box::new(ListNode::new(val));
        node.next = next_node;
        *head = Some(node);
        self.size += 1;
    }

    fn delete_at_index(&mut self, mut index: i32) {
        if index < 0 || index >= self.size {
            return;
        }

        let mut head = &mut self.head;
        for _ in 0..index {
            head = &mut head.as_mut().unwrap().next;
        }

        let mut node = head.take();
        *head = node.unwrap().next;
        self.size -= 1;
    }
}

pub fn main() {
    let mut my_linked_list = MyLinkedList::new();
    my_linked_list.add_at_head(1);
    my_linked_list.add_at_tail(3);
    my_linked_list.add_at_index(1,2);
    my_linked_list.delete_at_index(2);


    println!("{:?}", my_linked_list);
}
