use std::vec;

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

fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    let mut exceed = 0;

    loop {
        if l1.is_none() && l2.is_none() {
            if exceed == 1 {
                *tail = Some(Box::new(ListNode::new(exceed)));
                tail = &mut tail.as_mut().unwrap().next;
            }
            break;
        }

        let val1 = if let Some(node) = l1 {
            l1 = node.next;
            node.val
        } else { 0 };

        let val2 = if let Some(node) = l2 {
            l2 = node.next;
            node.val
        } else { 0 };

        *tail = Some(Box::new(ListNode::new((val1 + val2 + exceed) % 10)));
        tail = &mut tail.as_mut().unwrap().next;
        exceed = (val1 + val2 + exceed) / 10;
    }

    head
}

pub fn main() {
    let l1 = [9,9,9,9,9,9,9].to_vec();
    let l2 = [9,9,9,9].to_vec();
    println!("{:?}", add_two_numbers(vec_to_list(l1), vec_to_list(l2)));
}
