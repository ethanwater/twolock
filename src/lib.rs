use std::sync::{Arc, Mutex};

struct Node<T> {
    //Maybe use Arc?
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: Default> Node<T> {
    fn new(value: T, next: Option<Arc<Mutex<Node<T>>>>) -> Self {
        Self {
            value: value,
            next: next,
        }
    }
}

struct Queue<T> {
    //Maybe use Arc?
    head: Arc<Mutex<Node<T>>>,
    tail: Arc<Mutex<Node<T>>>,
    h_lock: Mutex<T>,
    t_lock: Mutex<T>,
}

impl<T: Default> Queue<T> {
    fn initialize(&mut self) {
        // Allocate a free node
        // Make it the only node in the linked list # Both Head and Tail point to it
        // Locks are initially free
        let node = Node::new(T::default(), None);
        let arc_node = Arc::new(Mutex::new(node));

        self.head = Arc::clone(&arc_node);
        self.tail = Arc::clone(&arc_node);
        self.h_lock = Mutex::new(T::default());
        self.t_lock = Mutex::new(T::default());
    }

    fn enqueue(&mut self, value: T) {
        // Allocate a new node from the free list
        // Copy enqueued value into node
        // Set next pointer of node to NULL
        // Acquire T lock in order to access Tail
        // Link node at the end of the linked list
        // Swing Tail to node
        // Release T lock
        let node = Arc::new(Mutex::new(Node::new(value, None)));

        let _t_guard = self.t_lock.lock().unwrap();
        let mut tail_node = self.tail.lock().unwrap();
        tail_node.next = Some(Arc::clone(&node));
        *tail_node = std::mem::replace(&mut *node.lock().unwrap(), Node::new(T::default(), None));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut queue: Queue<i32> = Queue {
            head: Arc::new(Mutex::new(Node::new(0, None))),
            tail: Arc::new(Mutex::new(Node::new(0, None))),
            h_lock: Mutex::new(i32::default()),
            t_lock: Mutex::new(i32::default()),
        };

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        // verify the tail points to the last enqueued value
        let tail_node = queue.tail.lock().unwrap();
        assert_eq!(tail_node.value, 3);

        //TODO: verify the previous tail's next points to the new tail
        //let binding = queue.head.lock().unwrap();
        //let prev_tail_next = binding.next.as_ref().unwrap().lock().unwrap();
        //assert_eq!(prev_tail_next.value, 1);
    }
}
