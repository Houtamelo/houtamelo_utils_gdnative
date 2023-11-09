use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use crate::godot_panic;
use gdnative::prelude::*;

pub trait UnwrapManual<T> where T : GodotObject<Memory = ManuallyManaged> {
	fn unwrap_manual(&self) -> TRef<T, Shared>;
}

impl<T> UnwrapManual<T> for Option<Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn unwrap_manual(&self) -> TRef<T, Shared> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return obj_tref;
			}
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<Ref<T, Shared>>>();
		godot_panic!("Failed to unwrap object through sane assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapManual<T> for Option<&Ref<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	fn unwrap_manual(&self) -> TRef<T, Shared> {
		if let Some(obj_ref) = self {
			if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
				return obj_tref;
			}
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<&Ref<T, Shared>>>();
		godot_panic!("Failed to unwrap object through sane assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapManual<T> for Ref<T, Shared> where T : GodotObject<Memory = ManuallyManaged> {
    fn unwrap_manual(&self) -> TRef<T, Shared> {
        if let Some(obj_tref) = unsafe { self.assume_safe_if_sane() } {
            return obj_tref;
        }

        let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Ref<T, Shared>>();
		godot_panic!("Failed to unwrap object through sane assertion, type: {type_name}.\n {trace}");
    }
}

impl<T> UnwrapManual<T> for &Ref<T, Shared> where T : GodotObject<Memory = ManuallyManaged> {
    fn unwrap_manual(&self) -> TRef<T, Shared> {
        if let Some(obj_tref) = unsafe { self.assume_safe_if_sane() } {
            return obj_tref;
        }

        let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Ref<T, Shared>>();
		godot_panic!("Failed to unwrap object through sane assertion, type: {type_name}.\n {trace}");
    }
}

pub trait UnwrapRefCount<T> where T : GodotObject<Memory = RefCounted> {
	fn unwrap_refcount(&self) -> TRef<T, Shared>;
}

impl<T> UnwrapRefCount<T> for Option<Ref<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	fn unwrap_refcount(&self) -> TRef<T, Shared> {
		if let Some(obj_ref) = self {
			return unsafe { obj_ref.assume_safe() };
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<Ref<T, Shared>>>();
		godot_panic!("Failed to unwrap object through safe assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapRefCount<T> for Option<&Ref<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	fn unwrap_refcount(&self) -> TRef<T, Shared> {
		if let Some(obj_ref) = self {
			return unsafe { obj_ref.assume_safe() };
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<&Ref<T, Shared>>>();
		godot_panic!("Failed to unwrap object through safe assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapRefCount<T> for Ref<T, Shared> where T : GodotObject<Memory = RefCounted> {
    fn unwrap_refcount(&self) -> TRef<T, Shared> {
        return unsafe { self.assume_safe() };
    }
}

impl<T> UnwrapRefCount<T> for &Ref<T, Shared> where T : GodotObject<Memory = RefCounted> {
    fn unwrap_refcount(&self) -> TRef<T, Shared> {
        return unsafe { self.assume_safe() };
    }
}

pub trait UnwrapInstance<T> where T : NativeClass {
	fn unwrap_inst<'a, 'r>(&'r self) -> TInstance<'a, T, Shared> where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory>;
}

impl<T> UnwrapInstance<T> for Option<Instance<T>> where T : NativeClass {
	fn unwrap_inst<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
	                            where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
		if let Some(obj_ref) = self {
			return unsafe { obj_ref.assume_safe() };
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<Instance<T>>>();
		godot_panic!("Failed to unwrap object through safe assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapInstance<T> for Option<&Instance<T>> where T : NativeClass {
	fn unwrap_inst<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
	                            where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
		if let Some(obj_ref) = self {
			return unsafe { obj_ref.assume_safe() };
		}

		let trace = std::backtrace::Backtrace::capture();
		let type_name = std::any::type_name::<Option<&Instance<T>>>();
		godot_panic!("Failed to unwrap object through safe assertion, type: {type_name}.\n {trace}");
	}
}

impl<T> UnwrapInstance<T> for Instance<T> where T : NativeClass {
    fn unwrap_inst<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
                                where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
        return unsafe { self.assume_safe() };
    }
}

impl<T> UnwrapInstance<T> for &Instance<T> where T : NativeClass {
    fn unwrap_inst<'a, 'r>(&'r self) -> TInstance<'a, T, Shared>
                                where AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
        return unsafe { self.assume_safe() };
    }
}