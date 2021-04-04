////实现迭代器
//pub trait Iterator {
//	type Item;
//	fn next(&mut self) -> Option<Self::Item>;
//}

////要实现的迭代器：
//IntoIter => T
//Iter => &T
//IterMut => &mut T

pub struct List<T> {
	head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None }
	}

	pub fn push(&mut self, elem: T) {
		let node = Box::new(Node {
			elem: elem,
			next: self.head.take(),
		});
		self.head = Some(node);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.elem
		})
	}

	pub fn peek(&self) -> Option<&T> {
		//self.head.map(|node| {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| {
			&mut node.elem
		})
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut link = self.head.take(); //ptr = node
		while let Some(mut node) = link {
			link = node.next.take();    //ptr = node->next
		}
	}
}

//实现IntoIter
pub struct IntoIter<T>(List<T>);

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

//实现Iter
pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}

impl<T> List<T> {
	pub fn iter(&self) -> Iter<T> {
		//pub fn as_deref(&self) -> Option<&<T as Deref>::Target>
		//Converts from Option<T> (or &Option<T>) to Option<&T::Target>
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

//实现IterMut
pub struct IterMut<'a, T> {
	next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
	pub fn iter_mut(&mut self) -> IterMut<T> {
		//pub fn as_deref_mut(&mut self) -> Option<&mut <T as Deref>::Target>
		IterMut { next: self.head.as_deref_mut() }
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node|{
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
		let mut list: List<i8> = List::new();
		assert_eq!(list.pop(), None);

		list.push(1);
		list.push(2);
		list.push(3);

		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2));
		list.push(4);
		assert_eq!(list.pop(), Some(4));
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
		
		let mut list: List<String> = List::new();
		assert_eq!(list.pop(), None);
		list.push("hello".to_string());
		list.push("world".to_string());
		list.push("!".to_string());
		assert_eq!(list.pop(), Some("!".to_string()));
		assert_eq!(list.pop(), Some("world".to_string()));
		assert_eq!(list.pop(), Some("hello".to_string()));
	}

	#[test]
	fn peek() {
		let mut list = List::new();
		assert_eq!(list.peek(), None);
		assert_eq!(list.peek_mut(), None);

		list.push(1);
		list.push(2);
		list.push(3);

		assert_eq!(list.peek(), Some(&3));
		assert_eq!(list.peek_mut(), Some(&mut 3));
		list.peek_mut().map(|value| {
			*value = 100;
		});

		assert_eq!(list.peek(), Some(&100));
		assert_eq!(list.pop(), Some(100));
		assert_eq!(list.pop(), Some(2));
		assert_eq!(list.pop(), Some(1));
	}

	#[test]
	fn into_iter() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iter() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);
		
		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iter_mut() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);
		
		let mut iter_mut = list.iter_mut();	
		assert_eq!(iter_mut.next(), Some(&mut 3));
		assert_eq!(iter_mut.next(), Some(&mut 2));
		assert_eq!(iter_mut.next(), Some(&mut 1));
		assert_eq!(iter_mut.next(), None);
	}
}

