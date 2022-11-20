use std::mem;

use crate::IoTDevice;

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev: dev,
            left: None,
            right: None,
        }))
    }
}

pub struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl DeviceRegistry {
    pub fn new_empty() -> DeviceRegistry {
        DeviceRegistry {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;

        // Moves src into the referenced dest, returning the previous dest value.
        // use std::mem;
        // let mut v: Vec<i32> = vec![1, 2];
        // let old_v = mem::replace(&mut v, vec![3, 4, 5]);
        // assert_eq!(vec![1, 2], old_v);
        // assert_eq!(vec![3, 4, 5], v);

        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    pub fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                // small numerical_id means more priority
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_recursive(&self.root, numerical_id)
    }

    pub fn find_recursive(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_recursive(&n.left, numerical_id)
                } else {
                    self.find_recursive(&n.right, numerical_id)
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}
