pub trait Clamp01<T> {
	fn clamp01(self) -> T;
}

impl Clamp01<f32> for f32 {
	fn clamp01(self) -> f32 {
		return f32::clamp(self, 0.0, 1.0);
	}
}

impl Clamp01<f64> for f64 {
	fn clamp01(self) -> f64 {
		return f64::clamp(self, 0.0, 1.0);
	}
}