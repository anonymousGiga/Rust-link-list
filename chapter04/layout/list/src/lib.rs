use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
	head: Link<T>,
	tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
	prev: Link<T>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
