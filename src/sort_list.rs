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

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut n = 0;

    let mut curr = &head;
    while let Some(node) = curr {
        n += 1;
        curr = &node.next;
    }

    if n <= 1 {
        return head;
    }

    let mut i = 1;
    while i < n {
        let mut curr = head.take();
        let mut tail = &mut head;

        while curr.is_some() {
            let (left, rest) = split_at(curr, i);
            let (right, rest) = split_at(rest, i);
            curr = rest;
            *tail = merge(left, right);
            while tail.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
        }

        i *= 2;
    }

    head
}

fn split_at(mut head: Option<Box<ListNode>>, size: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut curr = &mut head;

    for _ in 0..size {
        if let Some(node) = curr {
            curr = &mut node.next;
        } else {
            break;
        }
    }

    let rest = curr.take();

    (head, rest)
}

fn merge(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    while left.is_some() && right.is_some() {
        if left.as_ref().unwrap().val < right.as_ref().unwrap().val {
            *tail = Some(Box::new(ListNode::new(left.as_ref().unwrap().val)));
            left = left.unwrap().next;
        } else {
            *tail = Some(Box::new(ListNode::new(right.as_ref().unwrap().val)));
            right = right.unwrap().next;
        }

        tail = &mut tail.as_mut().unwrap().next;
    }

    *tail = if left.is_some() { left } else { right };
    head
}

pub fn main() {
    let head = [4,2,3,1].to_vec();
    println!("{:?}", sort_list(vec_to_list(head)));
}
