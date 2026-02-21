use std::thread::current;

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

fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let (mut min, mut current, mut adjusted) = (-1, -1, false);
    let mut res = vec![i32::MAX,-1];

    let mut prev_need = head.as_ref().unwrap().val;
    head = head.unwrap().next;
    let mut need = head.as_ref().unwrap().val;
    head = head.unwrap().next;

    let mut i = 2;
    while let Some(node) = head {
        head = node.next;
        if (need < prev_need && need < node.val) || (need > prev_need  && need > node.val) {
            if !adjusted {
                min = i;
                current = i;
                adjusted = true;
            } else {
                res[0] = std::cmp::min(res[0], i - current);
                res[1] = i - min;
                current = i;
            }
        }

        prev_need = need;
        need = node.val;
        i += 1;
    }

    res[0] = if res[0] == i32::MAX { -1 } else { res[0] };
    res
}

pub fn main() {
    let head = [1,3,2,2,3,2,2,2,7].to_vec();
    println!("{:?}", nodes_between_critical_points(vec_to_list(head)));
}
