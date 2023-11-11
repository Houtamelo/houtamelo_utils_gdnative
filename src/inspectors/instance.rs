use gdnative::prelude::*;
use crate::inspectors::*;

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Option<Instance<T>>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe_mut, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Option<&Instance<T>>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe_mut, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for &Option<Instance<T>>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe_mut, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_assert_safe_mut, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe_mut, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for &Option<&Instance<T>>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Err(error) = unsafe { value.assume_safe().map_mut(closure) } {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_if_safe, type: {type_name}.\n {trace}");
				}
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			match unsafe { value.assume_safe().map_mut(closure) } {
				Ok(result) => {
					return Some(result);
				},
				Err(error) => {
					godot_dbg!(error);
					let trace = std::backtrace::Backtrace::force_capture();
					let type_name = std::any::type_name::<Instance<T>>();
					godot_error!("Error on map of map_assert_safe, type: {type_name}.\n {trace}");
				}
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Instance<T>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Err(error) = unsafe { self.assume_safe().map(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Err(error) = unsafe { self.assume_safe().map_mut(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_if_safe_mut, type: {type_name}.\n {trace}");
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Err(error) = unsafe { self.assume_safe().map(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Err(error) = unsafe { self.assume_safe().map_mut(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_assert_safe_mut, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map_mut(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe_mut, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		match unsafe { self.assume_safe().map(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_assert_safe, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map_mut(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe_mut, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}
}

impl<'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for &Instance<T>
where
	T::UserData: Map,
	T::UserData: MapMut,
	T: NativeClass,
	AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>
{
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Err(error) = unsafe { self.assume_safe().map(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_if_safe, type: {type_name}.\n {trace}");
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Err(error) = unsafe { self.assume_safe().map_mut(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_if_safe_mut, type: {type_name}.\n {trace}");
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Err(error) = unsafe { self.assume_safe().map(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_assert_safe, type: {type_name}.\n {trace}");
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U)
	where T: std::fmt::Debug {
		if let Err(error) = unsafe { self.assume_safe().map_mut(closure) } {
			godot_dbg!(error);
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Instance<T>>();
			godot_error!("Error on touch_assert_safe_mut, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map_mut(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe_mut, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		match unsafe { self.assume_safe().map(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_assert_safe, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}

	#[must_use]
	fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
		match unsafe { self.assume_safe().map_mut(closure) } {
			Ok(ok) => {
				return Some(ok);
			},
			Err(error) => {
				godot_dbg!(error);
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Instance<T>>();
				godot_error!("Error on map_if_safe_mut, type: {type_name}.\n {trace}");
				return None;
			}
		}
	}
}
