use gdnative::prelude::{Shared, ToVariant, VariantArray};

pub trait IntoSharedArray {
	fn to_shared_array(self) -> VariantArray<Shared>;
}

impl<T: ToVariant> IntoSharedArray for T {
	fn to_shared_array(self) -> VariantArray<Shared> {
		return VariantArray::from_iter([self.to_variant()]).into_shared();
	}
}