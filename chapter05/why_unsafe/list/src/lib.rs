pub struct List<'a, T> {
	head: Link<T>,
	tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<'a, T> List<'a, T> {
	pub fn new() -> Self {
		List { head: None, tail: None }
	}

	pub fn push(&'a mut self, elem: T) {
		let new_tail = Box::new(Node{
			elem: elem,
			next: None,
		});

		let tail = match self.tail.take() {
			Some(old_tail) => {
				old_tail.next = Some(new_tail);
				old_tail.next.as_deref_mut()
			}
			None => {
				self.head = Some(new_tail);
				self.head.as_deref_mut()
			}
		};

		self.tail = tail;
	}

	pub fn pop(&'a mut self) -> Option<T> {
		self.head.take().map(|node| {
			let head = *node;
			self.head = head.next;
			
			if self.head.is_none() {
				self.tail = None;
			}

			head.elem
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
	}
}
