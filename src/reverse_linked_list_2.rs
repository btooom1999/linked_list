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

fn vec_to_list<I>(nums: I) -> Option<Box<ListNode>>
where
    I: IntoIterator<Item = i32>,
{
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn reverse_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = list {
        list = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}

fn reverse_between(mut head: Option<Box<ListNode>>, l: i32, r: i32) -> Option<Box<ListNode>> {
    let mut i = 1;
    let mut left = None;
    let mut left_tail = &mut left;

    let mut mid = None;
    let mut mid_tail = &mut mid;

    let mut right = &head;
    while let Some(node) = right {
        if i < l {
            *left_tail = Some(Box::new(ListNode::new(node.val)));
            left_tail = &mut left_tail.as_mut().unwrap().next;
        } else if i <= r {
            *mid_tail = Some(Box::new(ListNode::new(node.val)));
            mid_tail = &mut mid_tail.as_mut().unwrap().next;
        } else {
            break;
        }

        i += 1;
        right = &node.next;
    }

    let mid = reverse_list(mid);
    *left_tail = mid;

    while let Some(node) = left_tail {
        left_tail = &mut node.next;
    }

    *left_tail = right.clone();
    left
}

pub fn main() {
    let head = [1,2,3,4,5].to_vec();
    let left = 2;
    let right = 4;
    println!("{:?}", reverse_between(vec_to_list(head), left, right));
}
