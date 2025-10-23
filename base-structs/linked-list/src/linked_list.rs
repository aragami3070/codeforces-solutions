
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

impl<T> LinkedList<T> {
    /// Creates a new [`LinkedList<T>`].
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    /// Returns the length of this [`LinkedList<T>`].
    pub fn len(&self) -> usize {
        self.len
    }

    /// Add new node in end of [`LinkedList<T>`].
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

        self.len += 1;
    }

    /// Add new node in start of [`LinkedList<T>`]
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();

        self.head = Some(new_node);
        self.len += 1;
    }

    /// Add new node after node by index
    ///
    /// # Panics
    /// Panics if index incorrect
    pub fn insert(&mut self, index: usize, value: T) {
        if index > self.len - 1 {
            panic!("Index out of list range");
        }

        let mut index = index;
        let mut new_node = Box::new(Node::new(value));
        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            // find needed node
            if index == 0 {
                // move others nodes to new next (node.next == None now)
                new_node.next = node.next.take();
                // set new node in list like next after node by index
                node.next = Some(new_node);
                break;
            }
            // go to next node
            current_node = &mut node.next;
            index -= 1;
        }
        self.len += 1;
    }

    /// Returns the pop back of this [`LinkedList<T>`].
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 1 {
            let head = self.head.take().unwrap();
            self.len -= 1;
            return Some(head.value);
        }

        let mut current_node = &mut self.head;
        while let Some(node) = current_node {
            if node.next.as_ref().is_some_and(|n| n.next.is_none()) {
                let next_node = node.next.take().unwrap();
                self.len -= 1;
                return Some(next_node.value);
            }

            current_node = &mut node.next;
        }
        None
    }

    /// Returns the pop front of this [`LinkedList<T>`].
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let head = self.head.take().unwrap();
        self.head = head.next;
        self.len -= 1;
        Some(head.value)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();

        while let Some(mut node) = current_node {
            current_node = node.next.take();
        }
    }
}
