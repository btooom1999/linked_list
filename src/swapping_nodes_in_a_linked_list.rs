#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None}
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
fn count(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    while let Some(node) = head {
        count += 1;
        head = &node.next;
    }

    count
}

fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let n = count(&head);
    let mut i = 1;

    let (mut left, mut right) = (0, 0);
    let mut list = &head;
    while let Some(node) = list {
        if i == k {
            left = node.val;
        }
        if i == n-k+1 {
            right = node.val;
        }

        list = &node.next;
        i += 1;
    }

    let mut list = head.as_mut();
    i = 1;
    while let Some(node) = list {
        if i == k {
            node.val = right;
        }
        if i == n-k+1 {
            node.val = left;
        }

        list = node.next.as_mut();
        i += 1;
    }

    head
}

pub fn main() {
    let head = [1,2,3,4,5,6].to_vec();
    let k = 2;
    println!("{:?}", swap_nodes(vec_to_list(head), k));
}

