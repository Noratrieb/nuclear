use std::ptr;
use std::sync::atomic::AtomicPtr;

pub struct LinkedList<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    next: AtomicPtr<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
}
