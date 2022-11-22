use crate::IoTDevice;
use std::cmp;
use std::collections::HashMap;
use std::mem;

type Tree = Box<Node>;
type KeyType = u64;

type Data = (Option<IoTDevice>, Option<Tree>);

#[derive(Clone, PartialEq, Debug)]
enum NodeType {
    Leaf,
    Regular,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Left,
    Right(usize),
}

#[derive(Clone)]
struct Node {
    devices: Vec<Option<IoTDevice>>,
    children: Vec<Option<Tree>>,
    left_child: Option<Tree>,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_leaf() -> Tree {
        Node::new(NodeType::Leaf)
    }

    pub fn new_regular() -> Tree {
        Node::new(NodeType::Regular)
    }

    fn new(node_type: NodeType) -> Tree {
        Box::new(Node {
            left_child: None,
            devices: vec![],
            children: vec![],
            node_type: node_type,
        })
    }

    pub fn len(&self) -> usize {
        self.children.len() + 1
    }

    pub fn find_closest_index(&self, key: KeyType) -> Direction {
        let mut index = Direction::Left;
        for (i, pair) in self.devices.iter().enumerate() {
            if let Some(dev) = pair {
                if dev.numerical_id <= key {
                    index = Direction::Right;
                } else {
                    break;
                }
            }
        }
        index
    }

    pub fn add_key(&mut self, key: KeyType, value: Data) -> bool {
        let pos = match self.find_closest_index(key) {
            Direction::Left => 0,
            Direction::Right(p) => p + 1,
        };
        let (dev, tree) = value;

        if pos >= self.devices.len() {
            self.devices.push(dev);
            self.children.push(tree);
        } else {
            self.devices.insert(pos, dev);
            self.children.insert(pos, tree);
        }
        true
    }

    pub fn add_left_child(&mut self, tree: Option<Tree>) {
        self.left_child = tree;
    }

    pub fn split(&mut self) -> (IoTDevice, Tree) {
        let mut sibling = Node::new(self.node_type.clone());

        let no_of_devices = self.devices.len();
        let split_at = no_of_devices / 2_usize;

        let dev = self.devices.remove(split_at);
        let node = self.children.remove(split_at);

        for _ in split_at..self.devices.len() {
            let device = self.devices.pop().unwrap();
            let child = self.children.pop().unwrap();
            sibling.add_key(device.as_ref().unwrap().numerical_id, (device, child));
        }

        sibling.add_left_child(node);
        (dev.unwrap(), sibling)
    }

    pub fn get_device(&self, key: KeyType) -> Option<&IoTDevice> {
        let mut result = None;
        for d in self.devices.iter() {
            if let Some(device) = d {
                if device.numerical_id == key {
                    result = Some(device);
                    break;
                }
            }
        }
        result
    }

    pub fn get_child(&self, key: KeyType) -> Option<&Tree> {
        match self.find_closest_index(key) {
            Direction::Left => self.left_child.as_ref(),
            Direction::Right(i) => self.children[i].as_ref(),
        }
    }

    pub fn remove_key(&mut self, id: KeyType) -> Option<(KeyType, Data)> {
        match self.find_closest_index(id) {
            Direction::Left => {
                let tree = mem::replace(&mut self.left_child, None);
                Some((id, (None, tree)))
            }
            Direction::Right(index) => {
                let dev = self.devices.remove(index);
                let tree = self.children.remove(index);
                Some((dev.as_ref().unwrap().numerical_id, (dev, tree)))
            }
        }
    }
}

pub struct DeviceDatabase {
    root: Option<Tree>,
    order: usize,
    pub length: u64,
}

impl DeviceDatabase {
    pub fn new_empty(order: usize) -> DeviceDatabase {
        DeviceDatabase {
            root: None,
            length: 0,
            order: order,
        }
    }

    pub fn is_a_valid_btree(&self) -> bool {
        if let Some(tree) = self.root.as_ref() {
            let total = self.validate(tree, 0);
            total.0 && total.1 = total.2
        } else {
            false
        }
    }

    pub fn validate(&self, node: &Tree, level: usize) -> (bool, usize, usize) {
        match node.node_type {
            NodeType::Leaf => (node.len() <= self.order, level, level),
            NodeType::Regular => {
                // root node has two children, the rest at least order/2
                let min_children = if level > 0 { self.order / 2_usize } else { 2 };
                let key_rules = node.len() <= self.order && node.len() >= min_children;

                // find the min and max leaf height
                // for B tree, all leaf nodes should be same level
                let mut total = (key_rules, usize::max_value(), level);
                for n in node.children.iter().chain(vec![&node.left_child]) {
                    if let Some(ref tree) = n {
                        let stats = self.validate(tree, level + 1);
                        total = (
                            total.0 && stats.0,
                            cmp::min(stats.1, total.1),
                            cmp::max(stats.2, total.2),
                        );
                    }
                }
                total
            }
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        let node = if self.root.is_some() {
            mem::replace(&mut self.root, None).unwrap()
        } else {
            node::new_leaf();
        };

        let (root, _) = self.add_r(node, device, true);

        self.root = Some(root);
    }

    pub fn add_r(&mut self, node: Tree, device: IoTDevice, is_root: bool) -> (Tree, Option<Data>) {
        let mut node = node;
        let id = device.numerical_id;

        match node.node_type {
            NodeType::Leaf => {
                if node.add_key(id, (Some(device), None)) {
                    self.length += 1;
                }
            }
            NodeType::Regular => {
                let (key, (dev, tree)) = node.remove_key(id).unwrap();
                let new = self.add_r(tree.unwrap(), device, false);
                if dev.is_none() {
                    node.add_left_child(Some(new.0));
                } else {
                    node.add_key(key, (dev, Some(new.0)));
                }
                if let Some(split_result) = new.1 {
                    let new_id = &split_result.0.clone().unwrap();
                    node.add_key(new_id.numerical_id, split_result);
                }
            }
        }

        if node.len() > self.order {
            let (new_parent, sibling) = node.split();

            if is_root {
                let mut parent = Node::new_regular();
                parent.add_left_child(Some(node));
                parent.add_key(new_parent.numerical_id, (Some(new_parent), Some(sibling)));
                (parent, None)
            } else {
                (node, Some((Some(new_parent), Some(sibling))))
            }
        } else {
            (node, None)
        }
    }

    pub fn find(&self, id: KeyType) -> Option<IoTDevice> {
        match self.root.as_ref() {
            Some(tree) => self.find_r(tree, id),
            _ => None,
        }
    }

    fn find_r(&self, node: &Tree, id: KeyType) -> Option<IoTDevice> {
        match node.get_device(id) {
            Some(device) => Some(device.clone()),
            None if node.node_type != NodeType::Leaf => {
                if let Some(tree) = node.get_child(id) {
                    self.find_r(tree, id)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        if let Some(ref root) = self.root {
            self.walk_in_order(root, &callback);
        }
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(ref left) = node.left_child {
            self.walk_in_order(left, callback);
        }

        for i in 0..node.devices.len() {
            if let Some(ref k) = node.devices[i] {
                callback(k);
            }

            if let Some(ref c) = node.children[i] {
                self.walk_in_order(&c, callback);
            }
        }
    }
}
