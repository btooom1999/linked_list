#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None}
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn remove_elements(mut list: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    while let Some(mut node) = list {
        if node.val != val {
            *tail = Some(Box::new(ListNode::new(node.val)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        list = node.next;
    }

    head
}

pub fn main() {
    let head = [1,2,6,3,4,5,6].to_vec();
    let val = 6;
    println!("{:?}", remove_elements(vec_to_list(head), val));
}
