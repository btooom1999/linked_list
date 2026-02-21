use std::collections::HashSet;

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

fn modified_list(nums: Vec<i32>, mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let hashset = nums.into_iter().collect::<HashSet<_>>();

    let mut head = None;
    let mut tail = &mut head;
    while let Some(node) = list {
        list = node.next;
        if !hashset.contains(&node.val) {
            *tail = Some(Box::new(ListNode::new(node.val)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    head
}

pub fn main() {
    let nums = [1,2,3].to_vec();
    let head = [1,2,3,4,5].to_vec();
}
