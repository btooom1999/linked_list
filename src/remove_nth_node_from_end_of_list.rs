#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
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
fn count(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    while let Some(node) = head {
        head = &node.next;
        count += 1;
    }

    count
}

fn remove_nth_from_end(mut list: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let count = count(&list);
    let mut n = count - n;

    let mut head = None;
    let mut tail = &mut head;
    while let Some(node) = list {
        list = node.next;
        if n != 0 {
            *tail = Some(Box::new(ListNode::new(node.val)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        n -= 1;
    }

    head
}

pub fn main() {
    let head = [1,2,3,4,5].to_vec();
    let n = 2;
    println!("{:?}", remove_nth_from_end(vec_to_list(head), n));
}
