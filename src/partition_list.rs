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

fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();

    while let Some(node) = head {
        head = node.next;
        if node.val < x {
            stack1.push(node.val);
        } else {
            stack2.push(node.val);
        }
    }

    let mut head = None;
    let mut tail = &mut head;
    for &num in [stack1, stack2].iter().flatten() {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

pub fn main() {
    let head = [1,4,3,2,5,2].to_vec();
    let x = 3;
    println!("{:?}", partition(vec_to_list(head), x))
}
