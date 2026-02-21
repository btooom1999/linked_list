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

fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for _ in 0..a {
        if let Some(node) = list1 {
            *tail = Some(Box::new(ListNode::new(node.val)));
            tail = &mut tail.as_mut().unwrap().next;
            list1 = node.next;
        } else {
            break;
        }
    }

    for _ in a..=b {
        if let Some(node) = list1 {
            list1 = node.next;
        } else {
            break;
        }
    }

    while let Some(node) = list2 {
        list2 = node.next;

        *tail = Some(Box::new(ListNode::new(node.val)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    while let Some(node) = list1 {
        list1 = node.next;

        *tail = Some(Box::new(ListNode::new(node.val)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

pub fn main() {
    let list1 = [10,1,13,6,9,5].to_vec();
    let a = 3;
    let b = 4;
    let list2 = [1000000,1000001,1000002].to_vec();
    println!("{:?}", merge_in_between(vec_to_list(list1), a, b, vec_to_list(list2)));
}
