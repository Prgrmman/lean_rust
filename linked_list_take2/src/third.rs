use std::rc::Rc;

/* This time we will be doing a persistent list:
More info here: https://en.wikipedia.org/wiki/Persistent_data_structure 
*/
pub struct List<T> {
    head: Link<T>,
}

// Rc is like a reference counted box
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