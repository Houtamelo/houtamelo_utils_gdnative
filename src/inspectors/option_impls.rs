use crate::inspectors::*;

impl<T> SomeInspector<T> for Option<T> {
	fn touch_if_some(&self, closure: impl FnOnce(&T)) {
		if let Some(value) = self {
			closure(value);
		}
	}

	fn touch_assert_some(&self, closure: impl FnOnce(&T)) where T: std::fmt::Debug {
		if let Some(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<T>>();
			godot_warn!("touch_assert_some was called with none, type: {type_name}.\n {trace}");
		}
    }
}

impl<T> SomeInspector<T> for Option<&T> {
	fn touch_if_some(&self, closure: impl FnOnce(&T)) {
		if let Some(value) = self {
			closure(value);
		}
	}

	fn touch_assert_some(&self, closure: impl FnOnce(&T)) where T : std::fmt::Debug {
		if let Some(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&T>>();
			godot_warn!("touch_assert_some was called with none, type: {type_name}.\n {trace}");
		}
    }
}

impl<T> SomeMutInspector<T> for Option<T> {
	fn touch_if_some_mut(&mut self, closure: impl FnOnce(&mut T)) {
		if let Some(value) = self {
			closure(value);
		}
	}

	fn touch_assert_some_mut(&mut self, closure: impl FnOnce(&mut T)) where T : std::fmt::Debug {
		if let Some(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<T>>();
			godot_warn!("touch_assert_some_mut was called with none, type: {type_name}.\n {trace}");
		}
    }
}

impl <T> GodotManualSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = ManuallyManaged>  {
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}


	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		}
		else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
    }

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		}
		else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
    }
}

impl <T> GodotManualSomeInspector<T> for Option<&Ref<T>> where T: GodotObject<Memory = ManuallyManaged>  {
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			}
		}
	}


	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				closure(value);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("touch_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		}
		else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("touch_assert_sane was called with none, type: {type_name}.\n {trace}");
		}
    }

	#[must_use]
	fn map_if_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(value));
			}
		}

		return None;
	}

	#[must_use]
	fn map_assert_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug {
		if let Some(value) = self {
			if let Some(value) = unsafe { value.assume_safe_if_sane() } {
				return Some(closure(value));
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Ref<T>>>();
				godot_warn!("map_assert_sane was called with insane, type: {type_name}.\n {trace}");
			}
		}
		else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Ref<T>>>();
			godot_warn!("map_assert_sane was called with none, type: {type_name}.\n {trace}");
		}

		return None;
    }
}

impl <T> GodotRefCountedSomeInspector<T> for Option<Ref<T>> where T: GodotObject<Memory = RefCounted>  {
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			closure(unsafe { value.assume_safe() });
		}
	}


	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug {
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
	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug {
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


impl <T> GodotRefCountedSomeInspector<T> for Option<&Ref<T>> where T: GodotObject<Memory = RefCounted>  {
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>)) {
		if let Some(value) = self {
			let value = unsafe { value.assume_safe() };
			closure(value);
		}
	}


	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug {
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

	fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug {
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

impl <'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Option<Instance<T>>
	where T:: UserData:Map,
	      T:: UserData:MapMut,
	      T: NativeClass,
	      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			let _ = unsafe { value.assume_safe().map(closure) };
		}
	}

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self {
			let _ = unsafe { value.assume_safe().map_mut(closure) };
		}
	}

	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug {
        if let Some(value) = self {
			let _ = unsafe { value.assume_safe().map(closure) };
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
    }

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug {
        if let Some(value) = self {
			let _ = unsafe { value.assume_safe().map_mut(closure) };
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
    }

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
        if let Some(value) = self {
			if let Ok(result) = unsafe { value.assume_safe().map(closure) } {
				return Some(result);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Instance<T>>>();
				godot_error!("Error on map_mut of map_if_safe, type: {type_name}.\n {trace}");
			}
		}

		return None;
    }

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> where T : std::fmt::Debug {
        if let Some(value) = self {
			if let Ok(result) = unsafe { value.assume_safe().map_mut(closure) } {
				return Some(result);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<Instance<T>>>();
				godot_error!("Error on map_mut of map_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
    }
}

impl <TValue, TError> OkInspector<TValue> for Result<TValue, TError> {
	fn touch_if_ok(&self, closure: impl FnOnce(&TValue)) {
		if let Ok(value) = self {
			closure(value);
		}
	}

	fn touch_assert_ok(&self, closure: impl FnOnce(&TValue)) {
        if let Ok(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Result<TValue, TError>>();
			godot_warn!("touch_assert_ok was called with err, type: {type_name}.\n {trace}");
		}
    }
}

impl <TValue, TError> OkMutInspector<TValue> for Result<TValue, TError> {
	fn touch_if_ok_mut(&mut self, closure: impl FnOnce(&mut TValue)) {
		if let Ok(value) = self {
			closure(value);
		}
	}

	fn touch_assert_ok_mut(&mut self, closure: impl FnOnce(&mut TValue)) {
        if let Ok(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Result<TValue, TError>>();
			godot_warn!("touch_assert_ok_mut was called with err, type: {type_name}.\n {trace}");
		}
    }
}

impl NoneInspector for Option<()> {
	fn touch_if_none(&self, closure: impl FnOnce()) {
		if self.is_none() {
			closure();
		}
	}

	fn touch_assert_none(&self, closure: impl FnOnce()) {
        if self.is_none() {
			closure();
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			godot_warn!("touch_assert_none was called with some, type: void.\n {trace}");
		}
    }
}

impl <TValue, TError> ErrInspector<TError> for Result<TValue, TError> where TError: std::fmt::Display {
	fn touch_if_err(&self, closure: impl FnOnce(&TError)) {
		if let Err(error) = self {
			closure(error);
		}
	}


	fn touch_assert_err(&self, closure: impl FnOnce(&TError)) {
		if let Err(error) = self {
			closure(error);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Result<TValue, TError>>();
			godot_warn!("touch_assert_err was called with ok, type: {type_name}.\n {trace}");
		}
    }

	fn log_if_err(&self) {
		if let Err(error) = self {
			godot_error!("{}", error);
		}
	}
}



impl <'a, 'r, T> GodotInstanceSomeInspector<'a, 'r, T> for Option<&Instance<T>>
	where T:: UserData:Map,
	      T:: UserData:MapMut,
	      T: NativeClass,
	      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map(closure);
		}
	}


	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map(closure);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe was called with none, type: {type_name}.\n {trace}");
		}
    }

	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map_mut(closure);
		}
	}

	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug {
		if let Some(value) = self.assert_tinstance() {
			let _ = value.map_mut(closure);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("touch_assert_safe_mut was called with none, type: {type_name}.\n {trace}");
		}
	}

	#[must_use]
	fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> {
        if let Some(value) = self.assert_tinstance() {
			if let Ok(result) = value.map(closure) {
				return Some(result);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<&Instance<T>>>();
				godot_error!("Error on map_mut of map_if_safe, type: {type_name}.\n {trace}");
			}
		}

		return None;
    }

	#[must_use]
	fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> where T : std::fmt::Debug {
        if let Some(value) = self.assert_tinstance() {
			if let Ok(result) = value.map_mut(closure) {
				return Some(result);
			} else {
				let trace = std::backtrace::Backtrace::force_capture();
				let type_name = std::any::type_name::<Option<&Instance<T>>>();
				godot_error!("Error on map_mut of map_assert_safe, type: {type_name}.\n {trace}");
			}
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Option<&Instance<T>>>();
			godot_warn!("map_assert_safe was called with none, type: {type_name}.\n {trace}");
		}

		return None;
    }
}

impl <TValue, TError> OkInspector<TValue> for Result<&TValue, TError> {
	fn touch_if_ok(&self, closure: impl FnOnce(&TValue)) {
		if let Ok(value) = self {
			closure(value);
		}
	}

	fn touch_assert_ok(&self, closure: impl FnOnce(&TValue)) {
        if let Ok(value) = self {
			closure(value);
		} else {
			let trace = std::backtrace::Backtrace::force_capture();
			let type_name = std::any::type_name::<Result<&TValue, TError>>();
			godot_warn!("touch_assert_ok was called with err, type: {type_name}.\n {trace}");
		}
    }
}
