#![feature(test)]

mod bst;
mod heap;
mod rbtree;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
    pub path: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
        }
    }
}

impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}

#[derive(Clone, Debug)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub device: IoTDevice,
}

impl MessageNotification {
    pub fn new(device: IoTDevice, no_messages: u64) -> MessageNotification {
        MessageNotification {
            no_messages: no_messages,
            device: device,
        }
    }
}

impl PartialEq for MessageNotification {
    fn eq(&self, other: &MessageNotification) -> bool {
        self.device.eq(&other.device) && self.no_messages == other.no_messages
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::thread_rng;
    use rand::Rng;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use test::Bencher;

    const LIST_ITEMS: u64 = 1_000;

    fn new_device_with_id_path(id: u64, path: impl Into<String>) -> IoTDevice {
        IoTDevice::new(id, format!("address is {:?}", id), path)
    }

    fn new_device_with_id(id: u64) -> IoTDevice {
        new_device_with_id_path(id, "")
    }

    fn new_notification_with_id(id: u64, no_messages: u64) -> MessageNotification {
        let dev = new_device_with_id(id);
        MessageNotification::new(dev, no_messages)
    }

    // BST tests
    #[bench]
    fn bench_unsorted_insert_bst_find(b: &mut Bencher) {
        let mut tree = bst::DeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("not found");
        });
    }

    #[bench]
    fn bench_sorted_insert_bst_find(b: &mut Bencher) {
        let mut tree = bst::DeviceRegistry::new_empty();

        let items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);

        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("not found")
        });
    }

    #[test]
    fn binary_search_tree_add() {
        let mut tree = bst::DeviceRegistry::new_empty();
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(3));
        assert_eq!(tree.length, 2);
    }

    #[test]
    fn binary_search_tree_walk_in_order() {
        let len = 10;

        let mut tree = bst::DeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items.iter() {
            tree.add(item.clone());
        }

        assert_eq!(tree.length, len);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;

        // sort in descending order
        items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        // Consumes the RefCell, returning the wrapped value.
        assert_eq!(v.into_inner(), items);
    }

    #[test]
    fn binary_search_tree_find() {
        let mut tree = bst::DeviceRegistry::new_empty();

        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(3));

        assert_eq!(tree.find(100), None);
        assert_eq!(tree.find(4), Some(new_device_with_id(4)));
    }

    #[bench]
    fn bench_unsorted_insert_rbtree_find(b: &mut Bencher) {
        let mut tree = rbtree::BetterDeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }

        assert_eq!(tree.length, LIST_ITEMS);

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("not found");
        });
    }

    #[bench]
    fn bench_sorted_insert_rbtree_find(b: &mut Bencher) {
        let mut tree = rbtree::BetterDeviceRegistry::new_empty();

        for i in 0..LIST_ITEMS {
            tree.add(new_device_with_id(i));
        }

        assert_eq!(tree.length, LIST_ITEMS);
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("not found");
        });
    }

    #[test]
    fn binary_heap_add() {
        let mut heap = heap::MessageChecker::new_empty();

        heap.add(new_notification_with_id(1, 100));
        heap.add(new_notification_with_id(2, 200));
        heap.add(new_notification_with_id(3, 500));
        heap.add(new_notification_with_id(4, 40));
        assert_eq!(heap.length, 4);
    }

    #[test]
    fn binary_heap_pop() {
        let mut heap = heap::MessageChecker::new_empty();

        let a = new_notification_with_id(1, 40);
        let b = new_notification_with_id(2, 300);
        let c = new_notification_with_id(3, 50);
        let d = new_notification_with_id(4, 500);

        heap.add(a.clone());
        heap.add(b.clone());
        heap.add(c.clone());
        heap.add(d.clone());

        assert_eq!(heap.length, 4);

        assert_eq!(heap.pop(), Some(d));
        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), Some(a));
    }
}
