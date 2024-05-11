use util::prelude::*;
use crate::prelude::*;

pub fn load_prefab(path: &str) -> Result<Ref<PackedScene>> {
	ResourceLoader::godot_singleton()
		.load(path, "PackedScene", false)
		.map(|res| res.cast::<PackedScene>())
		.flatten()
		.ok_or_else(|| anyhow!("Failed to load prefab at path: {path}"))
}

pub fn spawn_prefab_as<T: GodotObject + SubClass<Node>>(path: &str) -> Result<TRef<T>> {
	let prefab_ref = load_prefab(path)?;
	let prefab = unsafe { prefab_ref.assume_safe() };

	unsafe {
		prefab.instance(0)
			  .ok_or_else(|| anyhow!("Failed to instance prefab at path: {path}"))?
			  .assume_safe()
			  .cast()
			  .ok_or_else(|| anyhow!("Failed to cast prefab instance to {}", type_name::<T>()))
	}
}

pub fn spawn_prefab_as_inst<T: NativeClass<Base: SubClass<Node>>>(path: &str) -> Result<TInstance<T>> {
	let node = spawn_prefab_as::<T::Base>(path)?;
	node.cast_instance()
	    .ok_or_else(|| anyhow!("Failed to cast prefab instance to {}", type_name::<T>()))
}

pub fn load_resource_as<T: GodotObject<Memory = RefCounted> + SubClass<Resource>>(path: &str) -> Result<Ref<T>> {
	ResourceLoader::godot_singleton()
		.load(path, T::class_name(), false)
		.ok_or_else(|| anyhow!("Failed to load resource at path: {path}"))
		.and_then(|res| {
			res.cast::<T>().ok_or_else(|| anyhow!("Failed to cast resource to {}", type_name::<T>()))
		})
}