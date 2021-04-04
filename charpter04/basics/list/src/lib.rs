use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;

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

impl<T> Node<T> {
	fn new(elem: T) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(Node {
			elem: elem,
			prev: None,
			next: None,
		}))
	}
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None, tail: None }
	}

	pub fn push_front(&mut self, elem: T) {
		let node = Node::new(elem);
		match self.head.take() {
			Some(head) => {
				head.borrow_mut().prev = Some(node.clone());
				node.borrow_mut().next = Some(head);
				self.head = Some(node);
			}

			None => {
				self.tail = Some(node.clone());
				self.head = Some(node);
			}
		}
	}

	pub fn push_back(&mut self, elem: T) {
		let node = Node::new(elem);
		match self.tail.take() {
			Some(tail) => {
				tail.borrow_mut().next = Some(node.clone());
				node.borrow_mut().prev = Some(tail);
				self.tail = Some(node);
			}

			None => {
				self.head = Some(node.clone());
				self.tail = Some(node);
			}
		}
	}

	pub fn pop_front(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			match node.borrow_mut().next.take() {
				Some(next) => {
					next.borrow_mut().prev.take();
					self.head = Some(next);
				}
				None => {
					self.tail.take();
				}
			}
			
			Rc::try_unwrap(node).ok().unwrap().into_inner().elem //对照手册好好理解
		})
	}

	pub fn pop_back(&mut self) -> Option<T> {
		self.tail.take().map(|node| {
			match node.borrow_mut().prev.take() {
				Some(prev) => {
					prev.borrow_mut().next.take();
					self.tail = Some(prev);
				}
				None => {
					self.head.take();
				}
			}
			Rc::try_unwrap(node).ok().unwrap().into_inner().elem
		})
	}

	//pub fn peek_front(&self) -> Option<&T> {
	pub fn peek_front(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|node| {
			//&node.borrow().elem
			//node.borrow()
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn peek_back(&self) -> Option<Ref<T>> {
		self.tail.as_ref().map(|node| {
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
		self.head.as_ref().map(|node| {
			RefMut::map(node.borrow_mut(), |node| &mut node.elem)
		})
	}

	pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
		self.tail.as_ref().map(|node| {
			RefMut::map(node.borrow_mut(), |node| &mut node.elem)
		})
	}
}

//实现迭代器
//Iter 不实现
//IterMut 不实现
//IntoIter
pub struct IntoIter<T> (List<T>);

impl<T> List<T> {
	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}
impl<T> Iterator for IntoIter<T> {
	type Item = T;	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop_front()
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> {
	fn next_back(&mut self) -> Option<T> {
		self.0.pop_back()
	}
}

#[cfg(test)]
mod tests {
	use super::List;

    #[test]
    fn basics() {
		let mut list = List::new();
		assert_eq!(list.pop_front(), None);

		list.push_front(1);
		list.push_front(2);
		list.push_front(3);

		assert_eq!(list.pop_front(), Some(3));
		assert_eq!(list.pop_front(), Some(2));
		
		list.push_front(4);
		list.push_front(5);

		assert_eq!(list.pop_front(), Some(5));
		assert_eq!(list.pop_front(), Some(4));
		assert_eq!(list.pop_front(), Some(1));
		assert_eq!(list.pop_front(), None);
		
		//----back-----
		assert_eq!(list.pop_back(), None);
		list.push_back(4);
		list.push_back(5);
		assert_eq!(list.pop_back(), Some(5));
		assert_eq!(list.pop_back(), Some(4));
		assert_eq!(list.pop_back(), None);
		
    }

	#[test]
	fn peek() {
		let mut list = List::new();
		assert!(list.peek_front().is_none());
		assert!(list.peek_back().is_none());
		assert!(list.peek_front_mut().is_none());
		assert!(list.peek_back_mut().is_none());

		list.push_front(1);
		list.push_front(2);
		list.push_front(3);

		assert_eq!(*list.peek_front().unwrap(), 3);
		assert_eq!(*list.peek_front_mut().unwrap(), 3);
		assert_eq!(*list.peek_back().unwrap(), 1);
		assert_eq!(*list.peek_back_mut().unwrap(), 1);
	}
	
	#[test]
	fn into_iter() {
		let mut list = List::new();
		list.push_front(1);
		list.push_front(2);
		list.push_front(3);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next_back(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next_back(), None);
		assert_eq!(iter.next(), None);
	}
}
