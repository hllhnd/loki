use std::cell::RefCell;
use std::rc::Rc;

use crate::executable::Executable;

pub struct Node {
	pub executable: Box<dyn Executable>,
	pub children:   Vec<Rc<RefCell<Node>>>,
}
