# gdrust_export_node_path_macro
Export any kind of Godot's Built-in Nodes as NodePath, most of the implementation was taken from the unmaintained gdrust repository: https://github.com/wyattjsmith1/gdrust

## Usage
### Instead of
```
#[derive(NativeClass)]
#[inherit(Node)] // Can also inherit any other built-in Node type
struct my_godot_script {
    #[property] exported_node_path: NodePath,
    exported_node: Option<Ref<Node>>, // Node can be replaced with any other godot built-in type
}

impl my_godot_script {
    fn new(_owner: &Node) -> Self {
        return Self { 
            path_exported_node: NodePath::default(),
            exported_node: None
        };
    }
}

#[methods]
impl my_godot_script {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.exported_node = Some(unsafe {
            _owner.get_node_as::<Node>(self.path_exported_node.new_ref()).unwrap().assume_shared()
        });
        //.. repeat for every other exported node as path
    }
}
```
### Use
```
#[gdrust(extends = Node)] // Can also extend any other built-in Node type
struct my_godot_script {
    #[export_node_path] exported_node: Option<Ref<Node>>, // Node can be replaced with any other godot built-in type
}

#[methods]
impl my_godot_script {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.grab_nodes_by_path(_owner); // you need to call this manually
        // do whatever else you want
    }
}
```

### Macro Expansion
```
#[derive(NativeClass)]
#[inherit(Node)]
struct my_godot_script {
    #[property] path_exported_node: NodePath,
    exported_node: Option<Ref<Node>>,
}

impl my_godot_script {
    fn new(_owner: &Node) -> Self {
        return Self { 
            exported_node_path: Default::default(),
            exported_node: Default::default()
        };
    }

    fn grab_nodes_by_path(&mut self, owner: &Node) {
        self.exported_node = Some(unsafe {
            owner.get_node_as::<gdnative::prelude::Node>(self.path_exported_node.new_ref()).unwrap().assume_shared()
        });
        //.. same is generated for any other fields marked with #[export_node_path]
    }
}

#[methods]
impl my_godot_script {
    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        self.grab_nodes_by_path(_owner);
    }
}
```
