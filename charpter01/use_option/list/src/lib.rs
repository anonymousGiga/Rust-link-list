pub struct List {
	head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
	elem: i32,
	next: Link,
}

impl List {
	pub fn new() -> Self {
		List {head: None}
	}

	pub fn push(&mut self, elem: i32) {
		let node = Box::new(Node {
			elem: elem,
			next: self.head.take(),
		});
		self.head = Some(node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		match self.head.take() {
			None => None,
			Some(node) => {
				self.head = node.next;
				Some(node.elem)
			}
		}
	}
}

impl Drop for List {
	fn drop(&mut self) {
		let mut link = self.head.take();
		while let Some(mut node) = link {
			link = node.next.take();
		}
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

		assert_eq!(list.pop(), Some(2));
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}
}
