struct SimpleLinkedNode<T> {
    data: T,
    next: Option<Box<SimpleLinkedNode<T>>>,
}
impl<T> SimpleLinkedNode<T> {
    fn new(data: T, next: Option<Box<SimpleLinkedNode<T>>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<SimpleLinkedNode<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }

        len
    }

    pub fn push(&mut self, element: T) {
        let new_head = SimpleLinkedNode::new(element, self.head.take());
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take()?;
        let head = *head;
        self.head = head.next;
        Some(head.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = Self::new();
        let mut current = self.head;
        while let Some(node) = current {
            reversed.push(node.data);
            current = node.next;
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut reversed = Vec::new();
        let mut current = linked_list.head;
        while let Some(node) = current {
            reversed.push(node.data);
            current = node.next;
        }
        reversed.reverse();
        reversed
    }
}
