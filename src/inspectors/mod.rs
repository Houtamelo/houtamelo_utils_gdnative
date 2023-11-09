pub mod option_impls;
pub mod option_ref_impls;

use gdnative::export::user_data::{Map, MapMut};
use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::prelude::*;
use super::tref_acquirer::*;

pub trait SomeInspector<T> {
	fn on_some(&self, closure: impl FnOnce(&T));
}

pub trait SomeMutInspector<T> {
	fn on_some_mut(&mut self, closure: impl FnOnce(&mut T));
}

pub trait GodotManualSomeInspector<T> where T: GodotObject<Memory = ManuallyManaged> {
	fn on_sane(&self, closure: impl FnOnce(TRef<T>));
	fn map_on_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>;
}

pub trait GodotRefCountedSomeInspector<T> where T: GodotObject<Memory = RefCounted> {
	fn on_safe(&self, closure: impl FnOnce(TRef<T>));
	fn map_on_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>;
}

pub trait GodotInstanceSomeInspector<'a, 'r, T>
	where T::UserData: Map,
	      T: NativeClass,
	      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	fn on_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U);
	fn on_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U);
}

pub trait OkInspector<T> {
	fn on_ok(&self, closure: impl FnOnce(&T));
}

pub trait OkMutInspector<T> {
	fn on_ok_mut(&mut self, closure: impl FnOnce(&mut T));
}

pub trait NoneInspector {
	fn on_none(&self, closure: impl FnOnce());
}

pub trait ErrInspector<T> {
	fn on_err(&self, closure: impl FnOnce(&T));
	fn report_on_err(&self);
}
