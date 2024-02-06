#![allow(unused)]
use std::{
    mem::replace,
    sync::{Arc, Mutex},
};
use tokio::sync;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Arc<Mutex<Node<T>>>>) -> Self {
        Self { value, next }
    }

    fn inner(self) -> Self {
        return self;
    }
}

struct Queue<T> {
    head: Arc<Mutex<Node<T>>>,
    tail: Arc<Mutex<Node<T>>>,
    h_lock: sync::Mutex<T>,
    t_lock: sync::Mutex<T>,
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
        self.h_lock = sync::Mutex::new(T::default());
        self.t_lock = sync::Mutex::new(T::default());
    }

    async fn enqueue(&mut self, value: T)
    where
        T: Clone,
    {
        // Allocate a new node from the free list
        // Copy enqueued value into node
        // Set next pointer of node to NULL
        // Acquire T lock in order to access Tail
        // Link node at the end of the linked list
        // Swing Tail to node
        // Release T lock
        let mut tail_node = self.tail.lock().unwrap();
        let mut head_node = self.head.lock().unwrap();

        if head_node.next.is_none() {
            let value_clone = value.clone();
            let node = Arc::new(Mutex::new(Node::new(value_clone, None)));
            head_node.next = Some(Arc::clone(&node));
        }

        let node = Arc::new(Mutex::new(Node::new(value, None)));
        if let Some(ref mut tail_next) = tail_node.next {
            tail_next.lock().unwrap().next = Some(Arc::clone(&node));
        }

        //TODO: mem::replace is a bit too sketchy
        //note: without this line of code, the head works fine, but the tails do not. and vice versa.
        //gonna take a break for now.
        *tail_node = std::mem::replace(&mut *node.lock().unwrap(), Node::new(T::default(), None));
        //*tail_node = ;
    }

    async fn dequeue(&mut self) -> Option<T>
    where
        T: Clone,
    {
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
        let head_node = self.head.lock().unwrap();

        if head_node.next.is_none() {
            return None;
        }

        if let Some(ref head_next) = head_node.next {
            let p_value = head_next.lock().unwrap().value.clone();
            let new_head = head_next.clone();

            drop(head_node);

            {
                let _ = &self.head;
                self.head = new_head.clone();
            }

            return Some(p_value);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::handle_alloc_error;
    use tokio::sync;

    use super::*;

    #[tokio::test]
    async fn test_queue() {
        let mut queue: Queue<i32> = Queue {
            head: Arc::new(Mutex::new(Node::new(0, None))),
            tail: Arc::new(Mutex::new(Node::new(0, None))),
            h_lock: sync::Mutex::new(i32::default()),
            t_lock: sync::Mutex::new(i32::default()),
        };

        {
            println!("\nqueue::init:");
            let head_node = queue.head.lock().unwrap();
            println!("{:?}", *head_node);
        }

        queue.enqueue(1).await;
        {
            println!("\nqueue::enqueue[1]:");
            let head_node = queue.head.lock().unwrap();
            let tail_value = queue.tail.lock().unwrap().value;

            println!("head: {:?}", *head_node);
            println!("Tail value: {}", tail_value);

            assert_eq!(head_node.value, 0);
            assert_eq!(tail_value, 1);
        }

        queue.enqueue(2).await;
        {
            println!("\nqueue::enqueue[2]:");
            let head_node = queue.head.lock().unwrap();
            let tail_value = queue.tail.lock().unwrap().value;

            println!("head: {:?}", *head_node);
            println!("Tail value: {}", tail_value);

            assert_eq!(head_node.value, 0);
            assert_eq!(tail_value, 2);
        }

        queue.enqueue(3).await;
        {
            println!("\nqueue::enqueue[3]:");
            let head_node = queue.head.lock().unwrap();
            let tail_value = queue.tail.lock().unwrap().value;

            println!("head: {:?}", *head_node);
            println!("Tail value: {}", tail_value);

            assert_eq!(head_node.value, 0);
            assert_eq!(tail_value, 3);
        }

        queue.dequeue().await;
        {
            println!("\nqueue::dequeue:");
            let head_node = queue.head.lock().unwrap();
            let tail_value = queue.tail.lock().unwrap().value;

            println!("head: {:?}", *head_node);
            println!("Tail value: {}\n", tail_value);

            assert_eq!(head_node.value, 1);
            assert_eq!(tail_value, 3);
        }
    }
}
