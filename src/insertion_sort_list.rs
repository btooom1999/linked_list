#[derive(Debug, Clone)]
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

fn insertion_sort_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    while let Some(node1) = list {
        list = node1.next;

        let mut tail = &mut head;
        while tail.is_some() && tail.as_ref().unwrap().val < node1.val {
            tail = &mut tail.as_mut().unwrap().next;
        }

        let mut node = Box::new(ListNode::new(node1.val));
        node.next = tail.take();
        *tail = Some(node);
    }

    head
}

pub fn main() {
    let head = [4,2,1,3].to_vec();
    println!("{:?}", insertion_sort_list(vec_to_list(head)));
}


