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

    fn pop(&mut self) -> Option<String> {
        if let Some(node) = self.head.clone() {
            let ret = node.borrow().value.clone();
            self.head = node.borrow().next.clone();

            if self.head.is_none() {
                self.tail = None;
            }

            return Some(ret);
        }

        None
    }

    fn add_after(&mut self, search_value: &str, value: &str) -> bool {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            if node.borrow().value == search_value {
                let new_node = Rc::new(RefCell::new(Node {
                    value: value.to_string(),
                    prev: Some(Rc::clone(&node)),
                    next: node.borrow().next.clone(),
                }));

                if new_node.borrow().next.is_none() {
                    self.tail = Some(Rc::clone(&new_node));
                } else {
                    if let Some(nn) = node.borrow().next.clone() {
                        nn.borrow_mut().prev = Some(Rc::clone(&new_node));
                    }
                }

                node.borrow_mut().next = Some(new_node);

                return true;
            }

            cur = node.borrow().next.clone();
        }

        false
    }

    fn print(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("'{}'", node.borrow().value);
            if node.borrow().next.is_some() {
                print!(" --> ")
            } else {
                println!();
            }
            cur = node.borrow().next.clone();
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push("One".to_string());
    list.push("Two".to_string());
    list.push("Three".to_string());
    list.print();

    if let Some(rm) = list.pop() {
        println!("RM: {}", rm);
    } else {
        println!("Empty list!")
    }

    list.print();

    list.add_after("Two", "3");

    list.print();
}
