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
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();

    loop {
        if l1.is_none() && l2.is_none() {
            break;
        }

        if let Some(node) = l1 {
            stack1.push(node.val);
            l1 = node.next;
        }

        if let Some(node) = l2 {
            stack2.push(node.val);
            l2 = node.next;
        }
    }

    let mut head = None;
    let mut exceed = 0;
    loop {
        if stack1.is_empty() && stack2.is_empty() {
            if exceed != 0 {
                let mut node = Box::new(ListNode::new(exceed));
                node.next = head;
                head = Some(node);
            }
            break;
        }

        let val = stack1.pop().unwrap_or(0) + stack2.pop().unwrap_or(0) + exceed;
        let mut node = Box::new(ListNode::new(val % 10));
        node.next = head;
        head = Some(node);
        exceed = val / 10
    }

    head
}

pub fn main() {
    let l1 = [7,2,4,3].to_vec();
    let l2 = [5,6,4].to_vec();
    println!("{:?}", add_two_numbers(vec_to_list(l1), vec_to_list(l2)));
}
