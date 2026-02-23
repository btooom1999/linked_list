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

#[inline]
fn length(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut n = 0;
    while let Some(node) = head {
        n += 1;
        head = &node.next;
    }

    n
}

fn split_list_to_parts(mut list: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut n = length(&list);
    let mut amount = (n as f32 / k as f32).ceil() as i32;

    let mut res = vec![None; k as usize];
    let mut head = None;
    let mut tail = &mut head;

    let mut i = 0;
    while let Some(node) = list {
        *tail = Some(Box::new(ListNode::new(node.val)));
        tail = &mut tail.as_mut().unwrap().next;
        list = node.next;

        amount -= 1;
        n -= 1;
        if amount == 0 {
            res[i] = head;
            i += 1;
            amount = (n as f32 / (k-i as i32) as f32).ceil() as i32;
            head = None;
            tail = &mut head;
        }
    }

    res
}

pub fn main() {
    let head = [1,2,3,4,5,6,7,8,9,10].to_vec();
    let k = 3;
    println!("{:?}", split_list_to_parts(vec_to_list(head), k));
}
