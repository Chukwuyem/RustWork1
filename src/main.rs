#[derive(Default)]

struct Node {
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

	fn set_left(&mut self, new_left: Option<Box<Node>>) -> Node {
		Node {data: self.data, left: new_left, right: self.right.unwrap()}
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
    println!("node4 val is {}", node4.print_data());

    let node3 = Node {data: -3, ..Default::default() };
}
