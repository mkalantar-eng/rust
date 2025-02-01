use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct IntList {
    head: Option<Rc<RefCell<Node>>>,
}