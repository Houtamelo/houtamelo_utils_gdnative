use util::prelude::*;

use crate::prelude::*;

pub trait InspectNode {
	fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T));
}

impl<TSelf: SubClass<Node>> InspectNode for &TSelf {
	fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T)) {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .map(|node| f(&node))
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}` or it is not of type `{}`",
				     owner.name(), node_path, type_name::<T>()))
			     .log_if_err();
		}
	}
}

pub trait MapNode {
	fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap>;
}

impl<TSelf: SubClass<Node>> MapNode for &TSelf {
	fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .map(|node| f(&node))
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}` or it is not of type `{}`",
				     owner.name(), node_path, type_name::<T>()))
		}
	}
}

pub trait TryGetNode {
	fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>>;
}

impl<TSelf: SubClass<Node>> TryGetNode for &TSelf {
	fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}` or it is not of type `{}`",
				     owner.name(), node_path, type_name::<T>()))
		}
	}
}

pub trait TryGetNodeInstance {
	fn try_get_node_inst<T>(self, node_path: &str) -> Result<TInstance<T>>
		where T: NativeClass,
		      T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged>;
}

impl<TSelf: SubClass<Node>> TryGetNodeInstance for &TSelf {
	fn try_get_node_inst<T>(self, node_path: &str) -> Result<TInstance<T>>
		where T: NativeClass,
		      T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as_instance::<T>(node_path)
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}` or it is not an instance of `{}`",
				     owner.name(), node_path, type_name::<T>()))
		}
	}
}

pub trait TryGetNodeScript {
	fn try_get_node_script<T: NativeClass<Base = Reference>>(self, node_path: &str) -> Result<Instance<T>>;
}

impl<TSelf: SubClass<Node>> TryGetNodeScript for &TSelf {
	fn try_get_node_script<T: NativeClass<Base = Reference>>(self, node_path: &str) -> Result<Instance<T>> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node(node_path)
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`",
				     owner.name(), node_path))
			     .and_then(|node| {
				     node.assume_safe()
					     .get_script()
				         .ok_or_else(|| anyhow!("Node `{}` at path `{}` does not have a script", owner.name(), node_path))
				         .and_then(|script| {
					         script.cast_instance()
					               .ok_or_else(|| anyhow!("Script on node `{}` at path `{}` is not of type `{}`", owner.name(), node_path, 
						               type_name::<T>()))
				         })
			     })
		}
	}
}