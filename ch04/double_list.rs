use std::cell::RefCell; 
use std::rc::Rc; 

// Rc is a pointer with shared ownership while RefCell provides interior mutability.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
type DoubleLink = Option<Rc<RefCell<Node>>>; 

#[derive(Clone)]
struct Node {
    value: String, 
    next: DoubleLink, 
    prev: DoubleLink, 
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(
            Node {
                value: value, 
                next: None, 
                prev: None, 
            }
        ))
    }
}

pub struct ListIterator {
    current: Link, 
}

impl ListIterator {
    fn new (start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at, 
        }
    }
}

impl Itertor for ListIterator {
    type Item = String; 
    fn next(&mut self) -> Option<String> {
        let current = &self.current; 
        let mut result = None; 
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow(); 
                result = Some(current.value.clone()); 
                current.next.clone()
            }, 
            None => None 
        };
        result 
    }
}

impl DoubleEnededIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current; 
        let mut result = None; 
        self.current = match current {
            let current = current.borrow(); 
            result = Some(current.value.clone()); 
        }, 
        None => None, 
    }; 
    result 
}

struct TransactionLog {
    head: DoubleLink, 
    tail: DoubleLink, 
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
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone()); 
                new.borrow_mut().prev = Some(old);  
            },
            None => self.head = Some(new.clone())
        }; 
        self.length += 1; 
        self.tail = Some(new); 
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(
            |head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None; 
                    self.head = Some(next); 
                } else {
                    self.tail.take(); 
                }
                self.length -= 1; 
                Rc::try_unwrap(head)
                    .ok()
                    .expect("sth is wrong")
                    .into_inner()
                    .value
        })
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone()); 
    }
}

impl IntoIerator for TransactionLog {
    type Item = String; 
    type IntoIter = ListIterator; 

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head); 
    }
}

fn main() {
    let mut log = TransactionLog::new_empty(); 
    log.append("iPhone13pro".to_string());
    log.append("iPhone14pro".to_string());
    for l in log {
        println!("iter {:?}", l.take());
    }
    for l in log.rev() {
        println!("back iter {:?}", l.take()); 
    }
}