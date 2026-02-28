#[derive(Debug, Clone)]
struct ImmutableListNode {
    val: i32,
    next: Option<Box<ImmutableListNode>>,
}

impl ImmutableListNode {
    fn new(val: i32) -> Self {
        ImmutableListNode { val, next: None }
    }

    fn get_next(&mut self) -> Option<&ImmutableListNode> {
        self.next.as_deref()
    }

    fn print_value(&self) {
        println!("{}", self.val);
    }
}

fn print_linked_list_in_reverse(node: ImmutableListNode) {
    fn helper(node: Option<ImmutableListNode>) {
        if let Some(mut node) = node {
            helper(node.get_next().cloned());
            node.print_value();
        }
    }

    helper(Some(node));
}

pub fn main() {
    let mut immutable_list_node = ImmutableListNode::new(5);
    let mut list_node_6 = ImmutableListNode::new(6);
    let mut list_node_7 = ImmutableListNode::new(7);
    let mut list_node_8 = ImmutableListNode::new(8);
    let mut list_node_9 = ImmutableListNode::new(9);

    list_node_8.next = Some(Box::new(list_node_9));
    list_node_7.next = Some(Box::new(list_node_8));
    list_node_6.next = Some(Box::new(list_node_7));
    immutable_list_node.next = Some(Box::new(list_node_6));

    print_linked_list_in_reverse(immutable_list_node);
}
