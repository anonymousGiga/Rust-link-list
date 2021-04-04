use std::rc::Rc;
pub struct List<T> {
	head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
