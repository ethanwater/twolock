use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

struct Node<T> {
    //Maybe use Arc?
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
    //Maybe use Arc?
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    h_lock: Mutex<T>,
    t_lock: Mutex<T>,
}

impl<T: Default> Queue<T> {
    fn initialize(&mut self) {
        // Allocate a free node
        // Make it the only node in the linked list # Both Head and Tail point to it
        // Locks are initially free
        let node = Rc::new(RefCell::new(Node::new(T::default(), None)));
        let shared_node = Rc::clone(&node);
        self.head = Some(node);
        self.tail = Some(shared_node);
        self.h_lock = Mutex::new(T::default());
        self.t_lock = Mutex::new(T::default());
    }

    fn enqueue(&mut self, value: T) {
        todo!();
        // Allocate a new node from the free list
        // Copy enqueued value into node
        // Set next pointer of node to NULL
        // Acquire T lock in order to access Tail 
        // Link node at the end of the linked list 
        // Swing Tail to node
        // Release T lock
        let node = Rc::new(RefCell::new(Node::new(value, None)));
        let mut tail_lock = self.t_lock.lock().unwrap();
        todo!()
    }

    fn dequeue(&mut self, p_value: *mut T) -> bool {
        todo!()
        //Acquire H lock in order to access Head # Read Head
        // Read next pointer
        // Is queue empty?
        // Release H lock before return # Queue was empty
        // Queue not empty. Read value before release # Swing Head to next node
        // Release H lock
        // Free node
        // Queue was not empty, dequeue succeeded
    }
}
