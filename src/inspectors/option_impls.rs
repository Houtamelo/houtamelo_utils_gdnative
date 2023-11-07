use crate::inspectors::*;

impl<T> SomeInspector<T> for Option<T> {
	fn on_some(&self, closure: impl FnOnce(&T)) {
		if let Some(value) = self {
			closure(value);
		}
	}
}

impl<T> SomeMutInspector<T> for Option<T> {
	fn on_some_mut(&mut self, closure: impl FnOnce(&mut T)) {
		if let Some(value) = self {
			closure(value);
		}
	}
}

impl <T> GodotManualSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = ManuallyManaged>  {
	fn on_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self.assert_tref_if_sane() {
			closure(value);
		}
	}

	fn map_on_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self.assert_tref_if_sane() {
			return Some(closure(value));
		}

		return None;
	}
}

impl <T> GodotRefCountedSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = RefCounted>  {
	fn on_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self.assert_tref() {
			closure(value);
		}
	}

	fn map_on_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self.assert_tref() {
			return Some(closure(value));
		}

		return None;
	}
}

impl <'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Option<Instance<T>>
	where T:: UserData:Map,
	      T:: UserData:MapMut,
	      T: NativeClass,
	      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	fn on_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map(closure);
		}
	}

	fn on_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map_mut(closure);
		}
	}
}

impl <TValue, TError> OkInspector<TValue> for Result<TValue, TError> {
	fn on_ok(&self, closure: impl FnOnce(&TValue)) {
		if let Ok(value) = self {
			closure(value);
		}
	}
}

impl <TValue, TError> OkMutInspector<TValue> for Result<TValue, TError> {
	fn on_ok_mut(&mut self, closure: impl FnOnce(&mut TValue)) {
		if let Ok(value) = self {
			closure(value);
		}
	}
}

impl NoneInspector for Option<()> {
	fn on_none(&self, closure: impl FnOnce()) {
		if self.is_none() {
			closure();
		}
	}
}

impl <TValue, TError> ErrInspector<TError> for Result<TValue, TError> where TError: std::fmt::Display {
	fn on_err(&self, closure: impl FnOnce(&TError)) {
		if let Err(error) = self {
			closure(error);
		}
	}

	fn report_on_err(&self) {
		if let Err(error) = self {
			godot_error!("{}", error);
		}
	}
}