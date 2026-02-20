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
    let mut reversed = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = reversed;
        reversed = Some(node);
    }

    reversed
}

fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while fast.is_some() && fast.unwrap().next.is_some() {
        slow = slow.unwrap().next.as_ref();
        fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
    }

    let mut reversed_head = reverse_list(slow.cloned());

    while let (Some(mut node1), Some(mut node2)) = (head, reversed_head) {
        if node1.val != node2.val {
            return false;
        }

        head = node1.next.take();
        reversed_head = node2.next.take();
    }

    true
}

pub fn main() {
    println!("{}", is_palindrome(vec_to_list(vec![1,2])));
}
