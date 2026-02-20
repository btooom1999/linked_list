use core::borrow;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut head: Option<Rc<RefCell<ListNode>>> = None;
    let mut tail: Option<Rc<RefCell<ListNode>>> = None;

    for num in nums {
        let node = Rc::new(RefCell::new(ListNode::new(num)));
        if let Some(mut t) = tail.clone() {
            t.borrow_mut().next = Some(node.clone());
        } else {
            head = Some(node.clone())
        }

        tail = Some(node);
    }

    head
}

fn add_cycle(mut list: Option<Rc<RefCell<ListNode>>>, mut pos: i32) -> Option<Rc<RefCell<ListNode>>> {
    if pos < 0 {
        return list;
    }

    let mut current = list.clone();
    while let Some(node) = current.clone() {
        if pos == 0 {
            break;
        }

        pos -= 1;
        current = node.borrow().next.clone();
    }

    if pos > 0 {
        return list;
    }

    while let Some(mut node) = list.clone() {
        if node.borrow().next.is_none() {
            node.borrow_mut().next = current.clone();
            break;
        }
        list = node.borrow().next.clone();
    }

    list
}

fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while let (Some(slow_node), Some(fast_node)) = (
        slow,
        fast
            .and_then(|v| v.borrow().next.clone())
            .and_then(|v| v.borrow().next.clone())
    )
    {
        slow = slow_node.borrow().next.clone();
        fast = fast_node.borrow().next.clone();

        if Rc::ptr_eq(&slow_node, &fast_node) {
            return true;
        }
    }

    false
}

pub fn main() {
    let head = add_cycle(vec_to_list(vec![3,2,0,0,-4]), 0);
    println!("{:?}", has_cycle(head));
}
