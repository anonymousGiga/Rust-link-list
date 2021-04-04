use std::ptr;

pub struct List<T> {
	head: Link<T>,
	tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None, tail: ptr::null_mut() }
	}

	pub fn push(&mut self, elem: T) {
		let mut new_tail = Box::new(Node {
			elem: elem,
			next: None,
		});

		let raw_tail: *mut _ = &mut *new_tail;
		
		if !self.tail.is_null() {
			unsafe {
				(*self.tail).next = Some(new_tail);
			}
		} else {
			self.head = Some(new_tail);
		}
		self.tail = raw_tail;
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			let head = *node;
			self.head = head.next;
			
			if self.head.is_none() {
				self.tail = ptr::null_mut();
			}
			head.elem
		})
	}
}

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
		self.0.pop()
	}
}

//Iter
pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}

impl<T> List<T> {
	pub fn iter(&self) -> Iter<T> {
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

//IterMut
pub struct IterMut<'a, T> {
	next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut { next: self.head.as_deref_mut() }
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;	
	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_deref_mut();
			&mut node.elem
		})
	}
}

#[cfg(test)]
mod tests {
	use super::List;

    #[test]
    fn basics() {
		let mut list = List::new();
		assert_eq!(list.pop(), None);

		list.push(1);
		list.push(2);
		list.push(3);
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), Some(2));
		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), None);
    }

	#[test]
	fn into_iter() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
	}

	#[test]
	fn iter() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), Some(&2));
	}

	#[test]
	fn iter_mut() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter_mut();
		assert_eq!(iter.next(), Some(&mut 1));
		assert_eq!(iter.next(), Some(&mut 2));
	}
}
