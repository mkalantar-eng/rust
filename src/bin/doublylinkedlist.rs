use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: String,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
        } else {
            self.tail = Some(Rc::clone(&new_node));
        }

        self.head = Some(new_node);
    }

    fn print(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("'{}'", node.borrow().value);
            if node.borrow().next.is_some() {
                print!(" --> ")
            }
            cur = node.borrow().next.clone();
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push("Hello, World!".to_string());
    list.push("Goodbye".to_string());
    list.push("Hi".to_string());
    list.print();
}
