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

#[inline]
fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();
    while let Some(node) = head {
        head = node.next;
        while let Some(&last) = stack.last() && last < node.val {
            stack.pop();
        }

        stack.push(node.val);
    }

    vec_to_list(stack)
}

pub fn main() {
    let head = [5,2,13,3,8].to_vec();
    println!("{:?}", remove_nodes(vec_to_list(head)));
}
