# gdrust_export_node_path_macro
Export any kind of Godot's Built-in Nodes as NodePath, most of the implementation was taken from the unmaintained gdrust repository: https://github.com/wyattjsmith1/gdrust

## Usage
### Instead of
```
#[derive(NativeClass)]
#[inherit(Node)] // Can also inherit any other built-in Node type
struct MyGodotScript {
    #[property] path_exported_node: NodePath,
    exported_node: Option<Ref<Node>>, // Node can be replaced with any other godot built-in type
    #[property] path_exported_instance: NodePath,
    exported_instance: Option<Instance<$UserType>> // replace $UserType with the native script you wish to export
}

impl MyGodotScript {
    fn new(_owner: &Node) -> Self {
        return Self { 
            path_exported_node: NodePath::default(),
            exported_node: None,
            path_exported_instance: NodePath::default(),
            exported_instance: None,
        };
    }
}

#[methods]
impl MyGodotScript {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.exported_node = Some(unsafe {
            _owner.get_node_as::<Node>(self.path_exported_node.new_ref()).unwrap().assume_shared()
        });
        self.exported_instance = Some(unsafe {
            _owner.get_node_as_instance::<PuddlesMiniGame>(self.path_exported_instance.new_ref()).unwrap().claim()
        });
        //.. repeat for every other exported node/instance path
    }
}
```
### Use
```
#[extends(Node)]// Can also inherit any other built-in Node type
struct MyGodotScript {
    #[export_node_path] exported_node: Option<Ref<Node>>, // Node can be replaced with any other godot built-in type
    #[export_instance_path] exported_instance: Option<Instance<PuddlesMiniGame>> // replace $UserType with the native script you wish to export
}

#[methods]
impl MyGodotScript {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.grab_nodes_by_path(_owner); // you must call this manually
    }
}
```

### Macro Expansion
```
#[derive(gdnative::prelude::NativeClass, Default)]
#[inherit(Node)]
struct MyGodotScript {
    #[property] path_exported_instance: gdnative::prelude::NodePath,
    #[property] path_exported_node: gdnative::prelude::NodePath,
    exported_node: Option<Ref<Node>>, // Node can be replaced with any other godot built-in type
    exported_instance: Option<Instance<PuddlesMiniGame>> // replace $UserType with the native script you wish to export
}

impl MyGodotScript {
    fn new(_owner: &Node) -> Self { Self::default() }

    fn grab_nodes_by_path(&mut self, owner: &Node) {
        self.exported_node = Some(unsafe {
            owner.get_node_as::<gdnative::prelude::Node>(self.path_exported_node.new_ref()).unwrap().assume_shared()
        });
        self.exported_instance = Some(unsafe {
            owner.get_node_as_instance::<PuddlesMiniGame>(self.path_exported_instance.new_ref()).unwrap().claim()
        });
    }
}

#[methods]
impl MyGodotScript {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.grab_nodes_by_path(_owner);
    }
}
```
