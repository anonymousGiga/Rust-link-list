use std::mem;
//use std::alloc::dealloc;

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

//实现Drop
//impl Drop for List{
//	fn drop(&mut self) {
//		self.head.drop();
//	}
//}
//
//impl Drop for Link {
//	fn drop(&mut self) {
//		match *self {
//			Link::Empty => {}
//			Link::More(ref mut node) => {
//				node.drop();
//			}
//		}
//	}
//}
//
//impl Drop for Box<Node> {
//	fn drop(&mut self) {
//		self.ptr.drop();
//		dealloc(self.ptr);
//	}
//}
//
//impl Drop for Node {
//	fn drop(&mut self) {
//		self.next.drop();
//	}
//}


impl Drop for List {
	fn drop(&mut self) {
		let mut link = mem::replace(&mut self.head, Link::Empty);
		while let Link::More(mut node) = link{
			link = mem::replace(&mut node.next, Link::Empty);
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
