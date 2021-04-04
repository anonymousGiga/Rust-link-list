use std::rc::Rc;

pub struct List<T> {
	head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head:None }
	}

	pub fn append(&mut self, elem: T) -> List<T> {
		List { head: Some(Rc::new(Node {
			elem: elem,
			next: self.head.clone(), //引用计数加1
		}))}
	}

	pub fn tail(&self) -> List<T> {
		List { head: self.head.as_ref().and_then(|node| {
			node.next.clone() //引用计数+1
		})}
	}

	pub fn head(&self) -> Option<&T> {
		self.head.as_ref().map(|node| { &node.elem })
	}
}

//IntoIter   xx:不会实现
//IterMut    xx:不会实现
//问题：为什么我们在此处只实现Iter？

//实现Iter
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

//实现Drop
impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut head = self.head.take();
		while let Some(node) = head {
			if let Ok(mut node) = Rc::try_unwrap(node) { //如果强引用计数为0
				head = node.next.take();
			} else {
				break;
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
		assert_eq!(list.head(), None);

		//let mut list = list.append(1);
		//let mut list = list.append(2);
		//let list = list.append(3);
		let list =list.append(1).append(2).append(3);
		assert_eq!(list.head(), Some(&3));

		let list = list.tail();
		assert_eq!(list.head(), Some(&2));

		let list = list.tail().tail();
		assert_eq!(list.head(), None);
	}

	#[test]
	fn iter() {
		let list = List::new().append(1).append(2).append(3);

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&1));
	}
	
}
