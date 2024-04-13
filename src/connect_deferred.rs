use crate::prelude::*;

pub trait ConnectFn {
	fn connect_fn(&self, signal: &str, target: &impl Inherits<Object>, fn_name: &str);
}

pub trait ConnectFnArgs {
	fn connect_fn_args(&self, signal: &str, target: &impl Inherits<Object>, fn_name: &str, args: VariantArray);
}

impl<T: Inherits<Object>> ConnectFn for T {
	fn connect_fn(&self, signal: &str, target: &impl Inherits<Object>, fn_name: &str) {
		let _self = unsafe { self.base().assume_safe() };
		_self.connect(signal, unsafe { target.base() }, fn_name, VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			 .log_if_err();
	}
}

impl<T: Inherits<Object>> ConnectFnArgs for T {
	fn connect_fn_args(&self, signal: &str, target: &impl Inherits<Object>, fn_name: &str, args: VariantArray) {
		let _self = unsafe { self.base().assume_safe() };
		_self.connect(signal, unsafe { target.base() }, fn_name, args, Object::CONNECT_DEFERRED)
			 .log_if_err();
	}
}