use crate::inspectors::*;

impl<T> SomeInspector<T> for Option<T> {
	fn on_some(&self, closure: impl FnOnce(&T))
	where T: std::fmt::Debug {
		if let Some(value) = self {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<T>>();
				godot_warn!("on_some was called with none, type: {type_name}.\n {trace}");
			}
		}
	}
}

impl<T> SomeMutInspector<T> for Option<T> {
	fn on_some_mut(&mut self, closure: impl FnOnce(&mut T)) {
		if let Some(value) = self {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<T>>();
				godot_warn!("on_some_mut was called with none, type: {type_name}.\n {trace}");
			}
		}
	}
}

impl <T> GodotManualSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = ManuallyManaged>  {
	fn on_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self.assert_tref_if_sane() {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("on_sane was called with none, type: {type_name}.\n {trace}");
			}
		}
	}

	fn map_on_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self.assert_tref_if_sane() {
			return Some(closure(value));
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_on_sane was called with none, type: {type_name}.\n {trace}");
			}
		}

		return None;
	}
}

impl <T> GodotRefCountedSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = RefCounted>  {
	fn on_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self.assert_tref() {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("on_safe was called with none, type: {type_name}.\n {trace}");
			}

		}
	}

	fn map_on_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self.assert_tref() {
			return Some(closure(value));
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_on_safe was called with none, type: {type_name}.\n {trace}");
			}
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
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Instance<T>>>();
				godot_warn!("on_safe was called with none, type: {type_name}.\n {trace}");
			}
		}
	}

	fn on_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map_mut(closure);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Instance<T>>>();
				godot_warn!("on_safe_mut was called with none, type: {type_name}.\n {trace}");
			}
		}
	}
}

impl <TValue, TError> OkInspector<TValue> for Result<TValue, TError> {
	fn on_ok(&self, closure: impl FnOnce(&TValue)) {
		if let Ok(value) = self {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Result<TValue, TError>>();
				godot_warn!("on_ok was called with err, type: {type_name}.\n {trace}");
			}
		}
	}
}

impl <TValue, TError> OkMutInspector<TValue> for Result<TValue, TError> {
	fn on_ok_mut(&mut self, closure: impl FnOnce(&mut TValue)) {
		if let Ok(value) = self {
			closure(value);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Result<TValue, TError>>();
				godot_warn!("on_ok_mut was called with err, type: {type_name}.\n {trace}");
			}
		}
	}
}

impl NoneInspector for Option<()> {
	fn on_none(&self, closure: impl FnOnce()) {
		if self.is_none() {
			closure();
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				godot_warn!("on_none was called with some, type: void.\n {trace}");
			}
		}
	}
}

impl <TValue, TError> ErrInspector<TError> for Result<TValue, TError> where TError: std::fmt::Display {
	fn on_err(&self, closure: impl FnOnce(&TError)) {
		if let Err(error) = self {
			closure(error);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Result<TValue, TError>>();
				godot_warn!("on_err was called with ok, type: {type_name}.\n {trace}");
			}
		}
	}

	fn report_on_err(&self) {
		if let Err(error) = self {
			godot_error!("{}", error);
		} else {
			#[cfg(feature = "log")] {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Result<TValue, TError>>();
				godot_warn!("report_on_err was called with ok, type: {type_name}.\n {trace}");
			}
		}
	}
}