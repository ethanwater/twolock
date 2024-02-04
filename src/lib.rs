use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Default> Node<T> {
    fn new(value: T, next: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self {
            value: value,
            next: next,
        }
    }
}

struct Queue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    h_lock: Mutex<T>,
    t_lock: Mutex<T>,
}

impl<T: Default> Queue<T> {
    fn initialize(&mut self) {
        let node = Rc::new(RefCell::new(Node::new(T::default(), None)));
        let shared_node = Rc::clone(&node);
        self.head = Some(node);
        self.tail = Some(shared_node);
        self.h_lock = Mutex::new(T::default());
        self.t_lock = Mutex::new(T::default());
    }

    fn enqueue(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value, None)));
        let mut tail_lock = self.t_lock.lock().unwrap();

        if let Some(tail_ref) = Some(Box::new(tail_lock).as_mut()) {
            tail_ref.borrow_mut().next = Some(Rc::clone(&node));
        }
    }
}
