#[derive(Default)]
//# [derive(Copy)]
pub struct Node {
	data: i32,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>
}
//option is used when the property could either be a type or None.
//if just `left: Node`, rust will panic because `recursive without indirection`

//impl Copy for Node { }

// impl Clone for Node {
// 	fn clone(&self) -> Node {
// 		*self
// 	}
// }

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

	fn swap_childs(&mut self) -> () {
		std::mem::swap(&mut self.left, &mut self.right);
	}

}

fn get_data(a_node: &Node) -> i32 {
	a_node.data
}


fn main() {
    println!("Hello, world!");

    let node4 = Node {data: 4, ..Default::default() };

    let mut node3 = Node {data: -3, ..Default::default() };
    node3.set_left(Some(Box::new(node4)));

    let node5 = Node {data: 5, ..Default::default() };
    node3.set_right(Some(Box::new(node5)));

    // We have a tree!?
    println!("node3 val is {}", get_data(&node3));

    let left = get_data(node3.left.as_ref().unwrap());
    //difference between & and as_ref() ==> &Option<T> vs Option<&T>
    println!("node3 left val is {}", left);

    let right = get_data(node3.right.as_ref().unwrap());
    println!("node3 right val is {}", right);

    node3.swap_childs();
    println!("nodes have been swapped");

    let left = get_data(node3.left.as_ref().unwrap());
    //difference between & and as_ref() ==> &Option<T> vs Option<&T>
    println!("node3 new left val is {}", left);

    let right = get_data(node3.right.as_ref().unwrap());
    println!("node3 new right val is {}", right);

}
