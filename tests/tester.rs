use gdnative::api::ProgressBar;
use gdnative::prelude::*;
use gdrust_export_node_path_macro::extends;

#[extends(Node)]
#[register_with(Self::register)]
pub struct MyInstanceTest {
	#[export_node_path] tested: Option<Ref<ProgressBar>>,
	#[export_instance_path] mine: Option<Instance<NativeScriptTest>>,
}

#[methods]
impl MyInstanceTest {
	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.grab_nodes_by_path(_owner);
	}

	fn register(_builder: &ClassBuilder<Self>) {
	}
}

#[extends(Node)]
pub struct NativeScriptTest {

}