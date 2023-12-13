use gdnative::api::ProgressBar;
use gdnative::prelude::*;
use gdrust_export_node_path_macro::extends;

#[extends(gdnative::prelude::Node)]
#[register_with(Self::register)]
pub struct MyInstanceTest {
	#[export_path] tested: Option<Ref<ProgressBar>>,
	#[export_path] tested_2: Option<Ref<gdnative::prelude::Node>>,
	#[export_path] mine: Option<Instance<NativeScriptTest>>,
	#[export_path] tested_vec: Vec<Ref<ProgressBar>>,
	#[export_path] tested_inst_vec: Vec<Instance<NativeScriptTest>>,
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
struct MyGodotScript {
	#[export_path] exported_node: Option<Ref<Node>>,
	#[export_path] exported_instance: Option<Instance<NativeScriptTest>>,
	#[export_path] vec_nodes: Vec<Ref<Node>>,
	#[export_path] vec_instances: Vec<Instance<NativeScriptTest>>,
}

#[methods]
impl MyGodotScript {
	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.grab_nodes_by_path(_owner);
	}
}

#[extends(Node)]
pub struct NativeScriptTest { }

/*#[derive(NativeClass)]
#[inherit(Node)]
struct MyGodotScript {
	#[property] path_exported_node: NodePath,
	exported_node: Option<Ref<Node>>,

	#[property] path_exported_instance: NodePath,
	exported_instance: Option<Instance<NativeScriptTest>>,

	#[property] paths_exported_nodes: Vec<NodePath>,
	vec_nodes: Vec<Ref<Node>>,

	#[property] paths_exported_instances: Vec<NodePath>,
	vec_instances: Vec<Instance<NativeScriptTest>>,
}

impl MyGodotScript {
	fn new(_owner: &Node) -> Self {
		return Self {
			path_exported_node: NodePath::default(),
			exported_node: None,
			path_exported_instance: NodePath::default(),
			exported_instance: None,
			paths_exported_nodes: Vec::new(),
			vec_nodes: Vec::new(),
			paths_exported_instances: Vec::new(),
			vec_instances: Vec::new(),
		};
	}
}

#[methods]
impl MyGodotScript {
	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.exported_node = Some(unsafe {_owner.get_node_as::<Node>(self.path_exported_node.new_ref()).unwrap().assume_shared()});
		self.exported_instance = Some(unsafe {_owner.get_node_as_instance::<NativeScriptTest>(self.path_exported_instance.new_ref()).unwrap().claim()});
		for path in self.paths_exported_nodes.iter() {
			self.vec_nodes.push(unsafe { _owner.get_node_as::<Node>(path.new_ref()).unwrap().assume_shared()});
		}
		for path in self.paths_exported_instances.iter() {
			self.vec_instances.push(unsafe { _owner.get_node_as_instance::<NativeScriptTest>(path.new_ref()).unwrap().claim()});
		}
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct NativeScriptTest { }

impl NativeScriptTest {
	fn new(_owner: &Node) -> Self {
		return Self {};
	}
}*/