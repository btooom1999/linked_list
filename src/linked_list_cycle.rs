// #[derive(Debug, PartialEq, Eq)]
// struct ListNode {
//     val: i32,
//     next: Option<*mut ListNode>,
// }

// impl ListNode {
//     fn new(val: i32) -> Self {
//         Self { val, next: None }
//     }
// }

// fn vec_to_list(nums: Vec<i32>) -> Option<*mut ListNode> {
//     let mut head: Option<*mut ListNode> = None;
//     let mut tail = &mut head;

//     for num in nums {
//         let mut node = Box::new(ListNode::new(num));
//         let mut raw_node: *mut _ = &mut *node;

//         *tail = Some(raw_node);
//         unsafe { tail = &mut (*tail.unwrap()).next; }

//         std::mem::forget(node);
//     }

//     head
// }

// fn add_cycle(mut head: Option<*mut ListNode>, pos: i32) -> Option<*mut ListNode> {
//     if pos < 0 {
//         return head;
//     }

//     let mut curr = head;
//     for _ in 0..pos {
//         if let Some(node) = curr {
//             unsafe { curr = (*node).next; }
//         } else {
//             return head;
//         }
//     }

//     let mut last = head;
//     unsafe {
//         while let Some(node) = last {
//             if (*node).next.is_some() {
//                 last = (*node).next;
//             } else {
//                 (*node).next = curr;
//                 break;
//             }
//         }
//     }

//     head
// }

// fn has_cycle(head: Option<*mut ListNode>) -> bool {
//     let mut slow = head;
//     let mut fast = head;
//     unsafe  {
//         while fast.is_some() && fast.is_some() && (*fast.unwrap()).next.is_some() {
//             slow = (*slow.unwrap()).next;
//             fast = (*(*fast.unwrap()).next.unwrap()).next;

//             if slow == fast {
//                 return true;
//             }
//         }
//     }

//     false
// }

#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn add_cycle(mut head: Option<Box<ListNode>>, pos: i32) -> Option<Box<ListNode>> {
    if pos < 0 {
        return head;
    }

    // tìm node tại vị trí pos
    let mut target: *mut ListNode = std::ptr::null_mut();
    let mut curr = head.as_mut();
    let mut idx = 0;
    while let Some(node) = curr {
        if idx == pos {
            target = &mut **node;
            break;
        }
        curr = node.next.as_mut();
        idx += 1;
    }

    if !target.is_null() {
        let mut last = head.as_mut();
        while let Some(node) = last {
            if node.next.is_none() {
                unsafe {
                    node.next = Some(Box::from_raw(target));
                }
                break;
            }
            last = node.next.as_mut();
        }
    }

    head
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    for num in nums {
        let mut node = Box::new(ListNode::new(num));
        *tail = Some(node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;
    unsafe  {
        while fast.is_some() && fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;

            if slow == fast {
                return true;
            }
        }
    }

    false
}

pub fn main() {
    let mut head = add_cycle(vec_to_list(vec![1,2]), 1);
    println!("{:?}", has_cycle(head));
}
