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
        let node = Box::new(ListNode::new(num));
        *tail = Some(node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    while list1.is_some() || list2.is_some() {
        match (list1.take(), list2.take()) {
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    list1 = node1.next.take();
                    list2 = Some(node2);

                    *tail = Some(Box::new(ListNode::new(node1.val)));
                    tail = &mut tail.as_mut().unwrap().next;
                } else {
                    list1 = Some(node1);
                    list2 = node2.next.take();

                    *tail = Some(Box::new(ListNode::new(node2.val)));
                    tail = &mut tail.as_mut().unwrap().next;
                }
            }
            (Some(mut node), None) => {
                list1 = node.next.take();

                *tail = Some(Box::new(ListNode::new(node.val)));
                tail = &mut tail.as_mut().unwrap().next;
            }
            (None, Some(mut node)) => {
                list2 = node.next.take();

                *tail = Some(Box::new(ListNode::new(node.val)));
                tail = &mut tail.as_mut().unwrap().next;
            }
            _ => unreachable!()
        }
    }

    head
}

pub fn main() {
    let list1 = [1,2,3].to_vec();
    let list2 = [1,2,4].to_vec();
    println!("{:?}", merge_two_lists(vec_to_list(list1), vec_to_list(list2)));
}
