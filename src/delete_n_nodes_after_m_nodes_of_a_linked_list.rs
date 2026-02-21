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

fn delete_nodes(mut list: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    let (mut count, mut should_delete) = (m, false);

    while let Some(mut node) = list {
        list = node.next.take();

        if count == 0 {
            count = if should_delete { m } else { n };
            should_delete = !should_delete;
        }

        if !should_delete {
            *tail = Some(Box::new(ListNode::new(node.val)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        count -= 1;
    }

    head
}

pub fn main() {
    // let head = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let head = vec![1,2,3,4,5,6,7,8,9,10,11];
    let m = 1;
    let n = 3;
    println!("{:?}", delete_nodes(vec_to_list(head), m, n));
}
