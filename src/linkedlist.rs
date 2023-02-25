struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl <T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None
        }
    }
}

pub struct LinkedList<T> {
    first: Option<Node<T>>,
    size: u32
} 

impl <T> LinkedList<T> {
    fn add(&mut self, value: T) -> bool {
        let new_node = Node { value, next: None };

        if let None = self.first {
            self.first = Some(new_node);
        } else {
            let mut parent: &mut Node<T> = self.first.as_ref().unwrap();
            let mut current = &self.first.as_ref().unwrap().next;

            while let Some(node) = current {
                parent = current.as_ref().unwrap().as_ref();
                current = &node.next;
            }

            parent.next = Some(Box::new(new_node));
        }

        true
    }
}
