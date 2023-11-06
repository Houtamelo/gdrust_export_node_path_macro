# gdrust_export_node_path_macro
Export any kind of Godot's Built-in Nodes as NodePath, most of the implementation was taken from the unmaintained gdrust repository: https://github.com/wyattjsmith1/gdrust

## Usage
### Instead of
```
#[derive(NativeClass)]
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
}
```
### Use
```
#[extends(Node)] // you can replace Node with any other Godot built-in node type
struct MyGodotScript {
    #[export_path] exported_node: Option<Ref<Node>>,                        // you can replace Node with any other Godot built-in node type
    #[export_path] exported_instance: Option<Instance<NativeScriptTest>>,   // replace NativeScriptTest with your own type
    #[export_path] vec_nodes: Vec<Ref<Node>>,                               // you can replace Node with any other Godot built-in node type
    #[export_path] vec_instances: Vec<Instance<NativeScriptTest>>,          // replace NativeScriptTest with your own type
}

#[methods]
impl MyGodotScript {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) { // replace Node with the extended type
        self.grab_nodes_by_path(_owner);  // you must call this manually, it replaces your old _ready() call
    }
}

#[extends(Node)]
pub struct NativeScriptTest { }
```

### Macro Expansion (manually formatted for readability)
```
#[derive(gdnative::prelude::NativeClass, Default)]
#[inherit(Node)]
struct MyGodotScript {
    #[property] path_vec_instances: Vec<gdnative::prelude::NodePath>,
    #[property] path_vec_nodes    : Vec<gdnative::prelude::NodePath>,
    #[property] path_exported_instance: gdnative::prelude::NodePath,
    #[property] path_exported_node    : gdnative::prelude::NodePath,
    exported_node: Option<Ref<Node>>,
    exported_instance: Option<Instance<NativeScriptTest>>,
    vec_nodes: Vec<Ref<Node>>,
    vec_instances: Vec<Instance<NativeScriptTest>>,
}

impl MyGodotScript {
    fn new(_owner: &Node) -> Self { Self::default() }

    fn grab_nodes_by_path(&mut self, owner: &Node) {
        self.exported_node = Some(unsafe { owner.get_node_as::<Node>(self.path_exported_node.new_ref()).unwrap().assume_shared() });
        self.exported_instance = Some(unsafe { owner.get_node_as_instance::<NativeScriptTest>(self.path_exported_instance.new_ref()).unwrap().claim() });

        for path in self.path_vec_nodes.iter() {
            self.vec_nodes.push(unsafe { owner.get_node_as::<Node>(path.new_ref()).unwrap().assume_shared() });
        }

        for path in self.path_vec_instances.iter() {
            self.vec_instances.push(unsafe { owner.get_node_as_instance::<NativeScriptTest>(path.new_ref()).unwrap().claim() });
        }
    }
}
```
