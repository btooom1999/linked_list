#[derive(Debug)]
struct ListNode {
    val: String,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: String) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug)]
struct BrowserHistory {
    url: Option<Box<ListNode>>,
    forward: Option<Box<ListNode>>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self { url: Some(Box::new(ListNode::new(homepage))), forward: None }
    }

    fn visit(&mut self, url: String) {
        let mut node = Box::new(ListNode::new(url));
        node.next = self.url.take();

        self.url = Some(node);
        self.forward = None;
    }

    fn back(&mut self, steps: i32) -> String {
        let mut head = &mut self.url;

        for _ in 0..steps {
            if let Some(node) = head && node.next.is_some() {
                let mut forward_node = Box::new(ListNode::new(node.val.clone()));
                forward_node.next = self.forward.take();
                self.forward = Some(forward_node);

                *head = node.next.take();
            } else {
                break;
            }
        }

        head.as_ref().unwrap().val.clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let mut head = &mut self.forward;

        for _ in 0..steps {
            if let Some(node) = head {
                let mut url_node = Box::new(ListNode::new(node.val.clone()));
                url_node.next = self.url.take();
                self.url = Some(url_node);

                *head = node.next.take();
            } else {
                break;
            }
        }

        self.url.as_ref().unwrap().val.clone()
    }
}

pub fn main() {
    let mut browser_history = BrowserHistory::new("leetcode.com".to_string());
    browser_history.visit("google.com".to_string());
    browser_history.visit("facebook.com".to_string());
    browser_history.visit("youtube.com".to_string());
    println!("{:?}", browser_history.back(1));
    println!("{:?}", browser_history.back(1));
    println!("{:?}", browser_history.forward(1));

    // println!("{:?}", browser_history);
}
