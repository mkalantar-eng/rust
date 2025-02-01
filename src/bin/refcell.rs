use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct IntList {
    head: Option<Rc<RefCell<Node>>>,
}

impl IntList {
    fn traverse(&self) {
        let mut current = self.head.clone();

        while let Some(node) = current {
            let node_ref = node.borrow();
            println!("{}", node_ref.value);
            current = node_ref.next.clone();
        }
    }
}

fn main() {
    // Example usage
    let node3 = Rc::new(RefCell::new(Node { value: 3, next: None }));
    let node2 = Rc::new(RefCell::new(Node { value: 2, next: Some(node3) }));
    let node1 = Rc::new(RefCell::new(Node { value: 1, next: Some(node2) }));

    let list = IntList { head: Some(node1) };
    list.traverse();
}