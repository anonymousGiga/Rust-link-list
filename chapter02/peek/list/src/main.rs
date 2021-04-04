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
}
