macro_rules! bound_main {
	($type_name:ty, $inner:ty) => {
		impl<const MIN: $inner, const MAX: $inner>  $type_name {
			pub fn new(inner_value: $inner) -> Self {
				return Self {
					inner_value: inner_value.clamp(MIN, MAX)
				};
			}

			pub fn get(&self) -> $inner { return self.inner_value; }

			pub fn set(&mut self, value: $inner) { self.inner_value = value.clamp(MIN, MAX); }
		}

		impl<const MIN: $inner, const MAX: $inner> Default for $type_name {
			fn default() -> Self { return Self::new(MIN); }
		}

		impl<const MIN: $inner, const MAX: $inner> std::hash::Hash for $type_name {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
		}
	};
}
