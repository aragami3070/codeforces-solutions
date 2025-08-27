#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}


impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}
