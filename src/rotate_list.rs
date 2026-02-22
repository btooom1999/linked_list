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

#[inline]
fn len(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut n = 0;
    while let Some(node) = head {
        n += 1;
        head = &node.next;
    }

    n
}

fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k == 0 {
        return head;
    }

    let n = len(&head);
    let mut k = n - k % n;
    if k == n {
        return head;
    }


    let mut left = None;
    let mut list = head.as_mut();
    while let Some(node) = list {
        k -= 1;
        if k == 0 {
            left = node.next.take();
            node.next = None;
        }
        list = node.next.as_mut();
    }

    let mut left_ref = left.as_mut();
    while let Some(node) = left_ref {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        left_ref = node.next.as_mut();
    }

    left
}

pub fn main() {
    let head = [1].to_vec();
    let k = 0;
    println!("{:?}", rotate_right(vec_to_list(head), k));
}
