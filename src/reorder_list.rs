use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[inline]
fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn reverse_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;

    while let Some(mut node) = list {
        list = node.next;
        node.next = head;
        head = Some(node);
    }

    head
}

fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let (mut slow, mut fast) = (head.as_ref(), head.as_ref());

    while slow.is_some() && fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = slow.as_ref().unwrap().next.as_ref();
        fast = fast.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
    }

    let mut reversed_head = reverse_list(slow.cloned());
    let mut i = 0;
    let mut removed_vals = VecDeque::new();
    let mut list = head.as_mut();
    while let Some(node) = list {
        list = node.next.as_mut();
        let temp_val = node.val;

        if i != 0 && i % 2 == 0 {
            node.val = removed_vals.pop_front().unwrap();
            removed_vals.push_back(temp_val);
        } else if i != 0 && i % 2 != 0 {
            node.val = reversed_head.as_ref().unwrap().val;
            reversed_head = reversed_head.unwrap().next;
            removed_vals.push_back(temp_val);
        }

        i += 1;
    }
}

pub fn main() {
    let mut head = vec_to_list([1,2,3,4,5,6,7].to_vec());
    reorder_list(&mut head);
    println!("{:?}", head)
}
