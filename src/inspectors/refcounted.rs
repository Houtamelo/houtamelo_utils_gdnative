use gdnative::prelude::*;
use crate::inspectors::*;

impl<T> GodotRefCountedSomeInspector<T> for Option<Ref<T>>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			closure(unsafe { value.assume_safe() });
		}
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		if let Some(value) = self {
			closure(unsafe { value.assume_safe() });
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			return Some(closure(unsafe { value.assume_safe() }));
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			return Some(closure(unsafe { value.assume_safe() }));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotRefCountedSomeInspector<T> for Option<&Ref<T>>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			closure(value);
		}
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Ref<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			return Some(closure(value));
		} else {
			return None;
		}
	}

	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			return Some(closure(value));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Ref<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotRefCountedSomeInspector<T> for &Option<Ref<T>>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			closure(unsafe { value.assume_safe() });
		}
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		if let Some(value) = self {
			closure(unsafe { value.assume_safe() });
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			return Some(closure(unsafe { value.assume_safe() }));
		}

		return None;
	}

	#[must_use]
	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			return Some(closure(unsafe { value.assume_safe() }));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotRefCountedSomeInspector<T> for &Option<&Ref<T>>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			closure(value);
		}
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Ref<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
	}

	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			return Some(closure(value));
		} else {
			return None;
		}
	}

	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			return Some(closure(value));
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Ref<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
	}
}

impl<T> GodotRefCountedSomeInspector<T> for Ref<T>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		let value = unsafe { self.assume_safe() };
		closure(value);
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		let value = unsafe { self.assume_safe() };
		closure(value);
	}

	#[must_use]
	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		let value = unsafe { self.assume_safe() };
		return Some(closure(value));
	}

	#[must_use]
	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		let value = unsafe { self.assume_safe() };
		return Some(closure(value));
	}
}

impl<T> GodotRefCountedSomeInspector<T> for &Ref<T>
where T: GodotObject<Memory = RefCounted>
{
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		let value = unsafe { self.assume_safe() };
		closure(value);
	}

	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>))
	where T: std::fmt::Debug {
		let value = unsafe { self.assume_safe() };
		closure(value);
	}

	#[must_use]
	fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		let value = unsafe { self.assume_safe() };
		return Some(closure(value));
	}

	#[must_use]
	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>
	where T: std::fmt::Debug {
		let value = unsafe { self.assume_safe() };
		return Some(closure(value));
	}
}
