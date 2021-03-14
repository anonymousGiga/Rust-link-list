use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
	pub fn new() -> Self {
		List { head: Link::Empty }	
	}

	pub fn push(&mut self, elem: i32) {
		let node = Box::new(Node {
			elem: elem,
			next: mem::replace(&mut self.head, Link::Empty),
		});	

		self.head = Link::More(node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		match mem::replace(&mut self.head, Link::Empty) {
			Link::Empty => None,
			Link::More(node) => {
				self.head = node.next;
				Some(node.elem)
			}
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
		list.push(3); //3->2->1->empty
		
		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2)); //1->empty

		list.push(4); //4->1->empty
		list.push(5); //5->4->1->empty
		assert_eq!(list.pop(), Some(5)); //4->1->empty
		assert_eq!(list.pop(), Some(4)); //1->empty
		assert_eq!(list.pop(), Some(1)); //empty
		assert_eq!(list.pop(), None); 
	}
}
