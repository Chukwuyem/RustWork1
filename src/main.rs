#[derive(Default)]

pub struct Node {
	data: i32,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>
}
//option is used when the property could either be a type or None.
//if just `left: Node`, rust will panic because `recursive without indirection`

impl Node {
	fn print_data(&self) -> i32 {
		self.data
	}

	fn default() -> Node { //function to help when instantiate node with no children
		Node {
			data: 0,
			left: None,
			right: None
		}
	}

	fn set_left(&mut self, new_left: Option<Box<Node>>) -> () {
       // If self.left was already a pointer, now we're losing it
       // (memory leak).
       self.left = new_left
		// Node {data: self.data, left: new_left, right: &self.right}
	}
	fn set_right(&mut self, new_right: Option<Box<Node>>) -> () {
       self.right = new_right
	}

	// fn return_left(&self) -> Option<&Self> {
	// 	if let Some(left) = self.left.as_ref() {
	// 		Some(&**left)
	// 	}
	// 	else {
	// 		None
	// 	}
	// }



	// fn return_right(&self) -> Node {
	// 	self.right
	// }

	// fn add_right(&self, new_right: Node) {
	// 	self.right = new_right
	// }
}

// fn new_node(new_data: i32) -> Node {
// 	Node {data: new_data, left: None, right: None}
// }

// fn new_node(new_data: i32, new_left: Node, new_right: Node) -> Node {
// 	Node {data: new_data, left: new_left, right: new_right}
// }

fn main() {
    println!("Hello, world!");

    // let node4 = new_node(4);
    // let node3 = new_node(3, node4);

    let node4 = Node {data: 4, ..Default::default() };

    let mut node3 = Node {data: -3, ..Default::default() };
    node3.set_left(Some(Box::new(node4)));

    let node5 = Node {data: 5, ..Default::default() };
    node3.set_right(Some(Box::new(node5)));

    // We have a tree!?
    println!("node3 val is {}", node3.print_data());
    let ld = node3.left.unwrap().data;
    println!("node3 left val is {}", ld);
    println!("node3 right val is {}", node3.right.unwrap().print_data());

    // Swap left and right values?
    // let tmp = &node3.left.unwrap().data;
    // node3.right.unwrap().data = tmp;
}
