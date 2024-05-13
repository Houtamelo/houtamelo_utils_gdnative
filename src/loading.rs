use util::prelude::*;

use crate::prelude::*;

pub fn load_prefab(path: &str) -> Result<Ref<PackedScene>> {
	ResourceLoader::godot_singleton()
		.load(path, "PackedScene", false)
		.and_then(|res| res.cast::<PackedScene>())
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

pub trait SpawnAs<'a> {
	fn spawn_as<T: SubClass<Node> + GodotObject<Memory = ManuallyManaged>>(self) -> Result<TRef<'a, T>>;
}

impl<'a, TSelf: SubClass<PackedScene>> SpawnAs<'a> for &'a TSelf {
	fn spawn_as<T: SubClass<Node> + GodotObject<Memory = ManuallyManaged>>(self) -> Result<TRef<'a, T>> {
		let scene: &PackedScene = self.upcast();

		scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
		     .ok_or_else(|| anyhow!("Failed to instance scene `{}`", scene.name()))
		     .and_then(|node| unsafe {
			     node.assume_safe()
			         .cast()
			         .ok_or_else(|| anyhow!("Node is not of type `{}`", T::class_name()))
		     })
	}
}

pub trait SpawnAsInst<'a> {
	fn spawn_as_inst<T>(self) -> Result<TInstance<'a, T>>
		where T: NativeClass,
		      T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged>;
}

impl<'a, TSelf: SubClass<PackedScene>> SpawnAsInst<'a> for &'a TSelf {
	fn spawn_as_inst<T>(self) -> Result<TInstance<'a, T>>
		where T: NativeClass,
		      T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged> {
		self.spawn_as::<T::Base>()
		    .and_then(|node| {
			    node.cast_instance()
			        .ok_or_else(|| anyhow!("Node is not an instance of type `{}`", type_name::<T>()))
		    })
	}
}