use gdnative::prelude::*;

use crate::inspectors::*;

impl<T> GodotManualSomeInspector<T> for Option<Ref<T>>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotManualSomeInspector<T> for &Option<Ref<T>>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotManualSomeInspector<T> for Option<&Ref<T>>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotManualSomeInspector<T> for &Option<&Ref<T>>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(&value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotManualSomeInspector<T> for Ref<T>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			closure(value);
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Ref<T>>();
			godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			return Some(closure(&value));
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			return Some(closure(&value));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Ref<T>>();
			godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotManualSomeInspector<T> for &Ref<T>
	where T: GodotObject<Memory = ManuallyManaged>
{
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			closure(value);
		}
	}

	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>))
	                     where T: std::fmt::Debug {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Ref<T>>();
			godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U> {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			return Some(closure(&value));
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(&T) -> U) -> Option<U>
	                      where T: std::fmt::Debug {
		if let Some(value) = unsafe { self.assume_safe_if_sane() } {
			return Some(closure(&value));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Ref<T>>();
			godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
		}

		return None;
	}
}
