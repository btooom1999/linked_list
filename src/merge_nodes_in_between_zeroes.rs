#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
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

fn merge_nodes(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    let mut sum = 0;

    list = list.unwrap().next;

    while let Some(node) = list {
        list = node.next;
        sum += node.val;
        if node.val == 0 {
            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
            sum = 0;
        }
    }

    head
}

pub fn main() {
    let head = [0,3,1,0,4,5,2,0].to_vec();
    println!("{:?}", merge_nodes(vec_to_list(head)));
}
