use std::cell::RefCell; 
use std::rc::Rc; 

// Rc is a pointer with shared ownership while RefCell provides interior mutability.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
type SingleLink = Option<Rc<RefCell<Node>>>; 

#[derive(Clone)]
struct Node {
    value: String, 
    next: SingleLink, 
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(
            Node {
                value: value, 
                next: None, 
            }
        ))
    }
}

struct TransactionLog {
    head: SingleLink, 
    tail: SingleLink, 
    pub length: u64
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None, 
            tail: None, 
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value); 
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }; 
        self.length += 1; 
        self.tail = Some(new); 
    }
}

fn main() {
    let mut log = TransactionLog::new_empty(); 
    log.append("iPhone".to_string());
}