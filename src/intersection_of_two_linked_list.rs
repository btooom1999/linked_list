#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>>  {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn get_intersection_node(
    mut head_a: Option<Box<ListNode>>,
    mut head_b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut p_a = head_a.as_ref();
    let mut p_b = head_b.as_ref();

    while p_a != p_b {
        p_a = match p_a {
            Some(node) => node.next.as_ref(),
            None => head_b.as_ref(),
        };
        p_b = match p_b {
            Some(node) => node.next.as_ref(),
            None => head_a.as_ref(),
        };
    }

    p_a.cloned()
}

pub fn main() {
    println!("{:?}", get_intersection_node(vec_to_list([6,4,1,8,4,5].to_vec()), vec_to_list([5,4,1,8,4,5].to_vec())))
}
