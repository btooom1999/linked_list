use std::collections::HashMap;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<*mut ListNode>,
    random: Option<*mut ListNode>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None, random: None }
    }
}

fn copy_random_list(mut list: Vec<(i32, Option<i32>)>) -> Option<*mut ListNode> {
    let mut hashmap = HashMap::new();
    let mut head = None;
    let mut tail = &mut head;

    unsafe {
        for (i, (num, _)) in list.iter().enumerate() {
            let mut node = Box::new(ListNode::new(*num));
            let raw_node: *mut _ = &mut *node;

            *tail = Some(raw_node);
            tail = &mut (*tail.unwrap()).next;
            hashmap.insert(i, raw_node);

            std::mem::forget(node);
        }
    }

    let mut curr = head;
    let mut i = 0;
    unsafe {
        while let Some(node) = curr {
            if let Some(pos) = list[i].1 {
                (*node).random = Some(*hashmap.get(&(pos as usize)).unwrap());
            }

            curr = (*node).next;
            i += 1;
        }
    }
    head
}

pub fn main() {
    let head = [(7, None), (13, Some(0)), (11, Some(4)), (10, Some(2)), (1, Some(0)), (1, Some(0))];
    unsafe  {
        let mut head = copy_random_list(head.into());
        while let Some(node) = head {
            println!("current: {:?} - val: {:?} - next: {:?} - random: {:?}", node, (*node).val, (*node).next, (*node).random);
            head = (*node).next;
        }
    }
}
