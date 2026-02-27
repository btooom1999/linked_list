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

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}

fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed = reverse(head);
    let mut exceed = 1;

    let mut res = None;
    while let Some(mut node) = reversed {
        reversed = node.next;

        let mut new_node = Box::new(ListNode::new((node.val + exceed) % 10));
        new_node.next = res;
        res = Some(new_node);
        exceed = (node.val + exceed) / 10;
    }

    if exceed != 0 {
        let mut new_node = Box::new(ListNode::new(exceed));
        new_node.next = res;
        res = Some(new_node);
    }

    res
}

pub fn main() {
    let head = [9,9,9];
    println!("{:?}", plus_one(vec_to_list(head.into())));
}
