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

fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = Box::new(ListNode::new(0));
    let mut tail = &mut res;

    let mut num = -101;
    let mut duplicate = 0;
    while head.is_some() {
        let val = head.as_ref().unwrap().val;
        head = head.unwrap().next;

        if num != val {
            if num != -101 && duplicate == 0 {
                tail.next = Some(Box::new(ListNode::new(num)));
                tail = tail.next.as_mut().unwrap();
            }
            num = val;
            duplicate = 0;
        } else {
            duplicate += 1;
        }
    }

    if duplicate == 0 && num != -101 {
        tail.next = Some(Box::new(ListNode::new(num)));
    }

    res.next
}

pub fn main() {
    let head = [1,2,3,3,4,4,5].to_vec();
    println!("{:?}", delete_duplicates(vec_to_list(head)));
}
