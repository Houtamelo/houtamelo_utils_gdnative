use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::prelude::*;

pub trait AssertSaneTRef<T> where T : GodotObject<Memory = ManuallyManaged> {
	fn assert_tref_if_sane(&self) -> Option<TRef<T, Shared>>;
}

impl<T> AssertSaneTRef<T> for Option<Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn assert_tref_if_sane(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return Some(obj_tref);
			}
		}

		godot_error!("Failed to get object through sane assertion, input: {self:?}");
		return None;
	}
}

impl<T> AssertSaneTRef<T> for Option<&Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn assert_tref_if_sane(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return Some(obj_tref);
			}
		}

		godot_error!("Failed to get object through sane assertion, input: {self:?}");
		return None;
	}
}

pub trait AssertSafeTRef<T> where T : GodotObject<Memory = RefCounted> {
	fn assert_tref(&self) -> Option<TRef<T, Shared>>;
}

impl<T> AssertSafeTRef<T> for Option<Ref<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	fn assert_tref(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		godot_error!("Failed to get object, input: {self:?}");
		return None;
	}
}

impl<T> AssertSafeTRef<T> for Option<&Ref<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	fn assert_tref(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		godot_error!("Failed to get object, input: {self:?}");
		return None;
	}
}

pub trait TrySaneTRef<T> where T : GodotObject<Memory = ManuallyManaged> {
	fn try_tref_if_sane(&self) -> Option<TRef<T, Shared>>;
}

impl<T> TrySaneTRef<T> for Option<Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn try_tref_if_sane(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return Some(obj_tref);
			}
		}

		return None;
	}
}

impl<T> TrySaneTRef<T> for Option<&Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn try_tref_if_sane(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return Some(obj_tref);
			}
		}

		return None;
	}
}

pub trait TrySafeTRef<T> where T : GodotObject<Memory = RefCounted> {
	fn try_tref(&self) -> Option<TRef<T, Shared>>;
}

impl<T> TrySafeTRef<T> for Option<Ref<T, Shared>> where T: GodotObject<Memory = RefCounted> {
	fn try_tref(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		return None;
	}
}

impl<T> TrySafeTRef<T> for Option<&Ref<T, Shared>> where T: GodotObject<Memory = RefCounted> {
	fn try_tref(&self) -> Option<TRef<T, Shared>> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		return None;
	}
}

pub trait AssertTInstance<T> where T : NativeClass {
	fn assert_tinstance<'a, 'r>(&'r self) -> Option<TInstance<'a, T, Shared>>
	                                                            where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>;
}

impl<T> AssertTInstance<T> for Option<Instance<T>> where T : NativeClass {
	fn assert_tinstance<'a, 'r>(&'r self) -> Option<TInstance<'a, T, Shared>>
	                            where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		godot_error!("Failed to get instance.");
		return None;
	}
}

impl<T> AssertTInstance<T> for Option<&Instance<T>> where T : NativeClass {
	fn assert_tinstance<'a, 'r>(&'r self) -> Option<TInstance<'a, T, Shared>>
	                            where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
		if let Some(obj_ref) = self {
			return Some(unsafe { obj_ref.assume_safe() });
		}

		godot_error!("Failed to get instance.");
		return None;
	}
}