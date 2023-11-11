pub mod option_impls;

use gdnative::export::user_data::{Map, MapMut};
use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::prelude::*;

pub trait SomeInspector<T> {
	fn touch_if_some(&self, closure: impl FnOnce(&T));
	fn touch_assert_some(&self, closure: impl FnOnce(&T)) where T : std::fmt::Debug;
}

pub trait SomeMutInspector<T> {
	fn touch_if_some_mut(&mut self, closure: impl FnOnce(&mut T));
	fn touch_assert_some_mut(&mut self, closure: impl FnOnce(&mut T)) where T : std::fmt::Debug;
}

pub trait GodotManualSomeInspector<T> where T: GodotObject<Memory = ManuallyManaged> {
	fn touch_if_sane(&self, closure: impl FnOnce(TRef<T>));
	fn touch_assert_sane(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug;
	#[must_use] fn map_if_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>;
	#[must_use] fn map_assert_sane<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug;
}

pub trait GodotRefCountedSomeInspector<T> where T: GodotObject<Memory = RefCounted> {
	fn touch_if_safe(&self, closure: impl FnOnce(TRef<T>));
	fn touch_assert_safe(&self, closure: impl FnOnce(TRef<T>)) where T : std::fmt::Debug;
	#[must_use] fn map_if_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U>;
	#[must_use] fn map_assert_safe<U>(&self, closure: impl FnOnce(TRef<T>) -> U) -> Option<U> where T : std::fmt::Debug;
}

pub trait GodotInstanceSomeInspector<'a, 'r, T>
	where T::UserData: Map,
	      T: NativeClass,
	      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	fn touch_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U);
	fn touch_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U);
	fn touch_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug;
	fn touch_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) where T : std::fmt::Debug;
	#[must_use] fn map_if_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>;
	#[must_use] fn map_if_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U>;
	#[must_use] fn map_assert_safe<U>(&'r self, closure: impl FnOnce(&T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> where T : std::fmt::Debug;
	#[must_use] fn map_assert_safe_mut<U>(&'r self, closure: impl FnOnce(&mut T, TRef<'_, <T as gdnative::prelude::NativeClass>::Base>) -> U) -> Option<U> where T : std::fmt::Debug;
}

pub trait OkInspector<T> {
	fn touch_if_ok(&self, closure: impl FnOnce(&T));
	fn touch_assert_ok(&self, closure: impl FnOnce(&T));
}

pub trait OkMutInspector<T> {
	fn touch_if_ok_mut(&mut self, closure: impl FnOnce(&mut T));
	fn touch_assert_ok_mut(&mut self, closure: impl FnOnce(&mut T));
}

pub trait NoneInspector {
	fn touch_if_none(&self, closure: impl FnOnce());
	fn touch_assert_none(&self, closure: impl FnOnce());
}

pub trait ErrInspector<T> {
	fn touch_if_err(&self, closure: impl FnOnce(&T));
	fn touch_assert_err(&self, closure: impl FnOnce(&T));
	fn log_if_err(&self);
}
