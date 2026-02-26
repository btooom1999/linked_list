use std::collections::HashMap;

#[derive(Debug)]
struct ListNode {
    val: (i32, i32),
    next: Option<Box<ListNode>>,
    prev: Option<*mut ListNode>,
}

impl ListNode {
    fn new(val: (i32, i32)) -> Self {
        Self { val, next: None, prev: None }
    }
}

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    hashmap: HashMap<i32, *mut ListNode>,
    head: Option<Box<ListNode>>,
    tail: Option<*mut ListNode>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as _;
        Self {
            capacity,
            head: None,
            tail: None,
            hashmap: HashMap::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&raw_ptr) = self.hashmap.get(&key) {
            unsafe {
                let value = (*raw_ptr).val;
                Self::put(self, key, value.1);
                value.1
            }
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        let mut node = Box::new(ListNode::new((key, value)));
        node.prev = self.tail;

        let mut raw_ptr: *mut _ = &mut *node;

        if let Some(tail) = self.tail {
            unsafe { (*tail).next = Some(node); }
        } else {
            self.head = Some(node);
        }

        self.tail = Some(raw_ptr);

        if let Some(rm_node) = self.hashmap.get_mut(&key) {
            unsafe {
                match ((**rm_node).prev, (**rm_node).next.take()) {
                    // delete middle node
                    (Some(mut prev_node), Some(mut next_node)) => {
                        next_node.prev = Some(prev_node);
                        (*prev_node).next = Some(next_node);
                    },
                    // delete first node
                    (None, Some(mut next_node)) => {
                        next_node.prev = None;
                        self.head = Some(next_node);
                    }
                    _ => unreachable!()
                }
            }

            *rm_node = raw_ptr;
        }

        self.hashmap.insert(key, raw_ptr);

        if self.hashmap.len() > self.capacity && let Some(rm_node) = self.hashmap.remove(&self.head.as_ref().unwrap().val.0) {
            unsafe {
                let mut next_node = (*rm_node).next.take().unwrap();
                next_node.prev = None;
                self.head = Some(next_node);
            }
        }
    }
}

pub fn main() {
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    lru_cache.put(1, 5);
    println!("{:?}", lru_cache.get(2));
    lru_cache.put(3, 3);

    unsafe {
        while let Some(node) = lru_cache.head {
            println!("{:?}", node);
            lru_cache.head = node.next;
        }
    }
}
