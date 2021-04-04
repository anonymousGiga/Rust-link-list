//#[derive(Debug)]
//pub struct Node {
//	elem: i32,
//	next: List,
//}
//
//#[derive(Debug)]
//pub enum List {
//	Empty,
//	More(Box<Node>),
//}
//
//fn main() {
//	let node2 = Node{elem: 2, next: List::Empty};
//	let node1 = Node{elem: 1, next: List::More(Box::new(node2))};
//	let list = Box::new(node1);
//	println!("{:?}", list);
//}

#[derive(Debug)]
struct Node {
	elem: i32,
	next: Link,
}

#[derive(Debug)]
enum Link {
	Empty,
	More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
	head: Link,
}

fn main() {
	let node2 = Node{elem: 2, next: Link::Empty};
	let node1 = Node{elem: 1, next: Link::More(Box::new(node2))};
	let list = List { head: Link::More(Box::new(node1))};
	println!("{:?}", list);
}
