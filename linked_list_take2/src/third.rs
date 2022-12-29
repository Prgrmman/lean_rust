use std::rc::Rc;

/* This time we will be doing a persistent list:
More info here: https://en.wikipedia.org/wiki/Persistent_data_structure 
*/
pub struct List<T> {
    head: Link<T>,
}

// Rc is like a reference counted box
// Note because Rc acts like a shared reference, we can't change the data type
// Notes on atomic reference counting: https://rust-unofficial.github.io/too-many-lists/third-arc.html
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None}
    }

    /* create a new list with elem in front */
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone()
            }))
        }
    }

    /* return a List with the first element "removed" */
    pub fn tail(&self) -> List<T> {
        // we clone the second element of the list (if it exists)
        // and_then is like map, except it allows us to work on a function that returns an Option type
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    /* return a reference to the first element */
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Drop for List<T> {
    /* Explanation:
    try_unwrap will attempt to move a value out of its box if it has exactly one reference.
    From there, if we are the last list that knows about a node, we can "take" it, and then allow it to go out of scope
    This also gives us a stopping condition: if we can't hoist the Node out of the box because we are not the last holder,
    then we should stop iterating as we are not the last reference holder
     */
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}

