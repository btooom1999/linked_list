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

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for n in nums {
        let new_node = Box::new(ListNode::new(n));
        *tail = Some(new_node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    println!("{:?}", reverse_list(vec_to_list(nums)));
}
