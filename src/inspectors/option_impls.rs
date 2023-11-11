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
