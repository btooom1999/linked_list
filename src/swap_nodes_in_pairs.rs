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

fn swap_pairs(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = list.as_mut();
    while let Some(node) = head {
        if let Some(next_node) = node.next.as_mut() {
            (node.val, next_node.val) = (next_node.val, node.val);
            head = next_node.next.as_mut();
        } else {
            break;
        }
    }

    list
}

pub fn main() {
    let head = [1,2,3,4].to_vec();
    println!("{:?}", swap_pairs(vec_to_list(head)));
}
