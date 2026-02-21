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

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = None;
    while let Some(mut node) = head {
        head = node.next;
        node.next = list;
        list = Some(node);
    }

    list
}

fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    let mut sum = 0;

    let (mut slow, mut fast) = (&head, &head);
    while slow.is_some() && fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    let mut reversed_head = reverse_list(slow.clone());
    while let Some(node) = reversed_head {
        sum = sum.max(node.val + head.as_ref().unwrap().val);

        reversed_head = node.next;
        head = head.unwrap().next;
    }

    sum
}

pub fn main() {
    let head = [5,4,2,1].to_vec();
    println!("{}", pair_sum(vec_to_list(head)));
}


