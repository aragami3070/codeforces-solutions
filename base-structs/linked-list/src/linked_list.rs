#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current_node = &mut self.head;

            while let Some(node) = current_node {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current_node = &mut node.next;
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();

        self.head = Some(new_node);
    }
}
