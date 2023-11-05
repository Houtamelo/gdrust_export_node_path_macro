use gdnative::prelude::*;
use gdnative::api::*;
use gdrust_export_node_path_macro::extends;

#[extends(Node)]
pub struct MyInstanceTest {
	#[export_node_path] tested: Option<Ref<Node2D>>,
	#[export_instance_path] mine: Option<Instance<NativeScriptTest>>,
}

#[methods]
impl MyInstanceTest {
	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.grab_nodes_by_path(_owner);
	}
}

#[extends(Node)]
pub struct NativeScriptTest {

}