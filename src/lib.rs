#![allow(unused)]
use std::{
    borrow::Borrow, mem::replace, ops::Deref, sync::{Arc, Mutex}
};

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: Default> Node<T> {
    fn new(value: T, next: Option<Arc<Mutex<Node<T>>>>) -> Self {
        Self { value, next }
    }

    fn inner(self) -> Self {
        return self
    }
}

struct Queue<T> {
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
        let new_node = Node::new(value, None);
        let node = Arc::new(Mutex::new(new_node));
    
        let mut tail_node = self.tail.lock().unwrap();
        let mut head_node = self.head.lock().unwrap();

        if let Some(ref mut tail_next) = tail_node.next {
            tail_next.lock().unwrap().next = Some(Arc::clone(&node));
        }
        if head_node.next.is_none() {
            head_node.next = Some(Arc::clone(&node));
        }
        //TODO: mem::replace is too sketchy
        //note: without this line of code, the head works fine, but the tails do not. and vice versa.
        //gonna take a break for now.
        *tail_node = std::mem::replace(&mut *node.lock().unwrap(), Node::new(T::default(), None));//*tail_node = ;  
    }

    fn dequeue(&mut self, p_value: *mut T) -> bool {
        todo!();
        // Acquire H lock in order to access Head
        // Read Head
        // Read next pointer
        // Is queue empty?
        // Release H lock before return # Queue was empty
        // Queue not empty. Read value before release # Swing Head to next node
        // Release H lock
        // Free node
        // Queue was not empty, dequeue succeeded

        let _h_guard = self.h_lock.lock().unwrap();
        let mut head_node = self.head.lock().unwrap();
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
    
        {
            let head_node = queue.head.lock().unwrap();
            println!("{:?}", *head_node);
        }
    
        queue.enqueue(1);
        {
            let head_node = queue.head.lock().unwrap();
            println!("{:?}", *head_node);
            let tail_value = queue.tail.lock().unwrap().value;
            println!("Tail value: {}", tail_value);
        }
    
        queue.enqueue(2);
        {
            let head_node = queue.head.lock().unwrap();
            println!("{:?}", *head_node);
            let tail_value = queue.tail.lock().unwrap().value;
            println!("Tail value: {}", tail_value);
        }
    
        queue.enqueue(3);
        {
            let head_node = queue.head.lock().unwrap();
            println!("{:?}", *head_node);
            let tail_value = queue.tail.lock().unwrap().value;
            println!("Tail value: {}", tail_value);
        }
        //TODO: verify the previous tail's next points to the new tail
    }
     
}
