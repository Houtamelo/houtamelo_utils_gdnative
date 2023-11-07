use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct BoundISize<const MIN: isize, const MAX: isize> {
	inner_value: isize
}

impl<const MIN: isize, const MAX: isize> BoundISize<MIN, MAX> {
	pub fn new(inner_value: isize) -> Self {
		return Self { inner_value: inner_value.clamp(MIN, MAX) };
	}

	pub fn get(&self) -> isize {
		return self.inner_value;
	}
}

impl<const MIN: isize, const MAX: isize> Default for BoundISize<MIN, MAX> {
	fn default() -> Self {
		return Self::new(0);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Add for BoundISize<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		return Self::new(self.inner_value + other.inner_value);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Add<isize> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn add(self, other: isize) -> Self {
		return Self::new(self.inner_value + other);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::AddAssign for BoundISize<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value + other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::AddAssign<isize> for BoundISize<MIN, MAX> {
	fn add_assign(&mut self, other: isize) {
		self.inner_value = (self.inner_value + other).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Sub for BoundISize<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		return Self::new(self.inner_value - other.inner_value);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Sub<isize> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: isize) -> Self {
		return Self::new(self.inner_value - other);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::SubAssign for BoundISize<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value - other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::SubAssign<isize> for BoundISize<MIN, MAX> {
	fn sub_assign(&mut self, other: isize) {
		self.inner_value = (self.inner_value - other).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Div for BoundISize<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		return Self::new(self.inner_value / other.inner_value);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Div<isize> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn div(self, other: isize) -> Self {
		return Self::new(self.inner_value / other);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::DivAssign for BoundISize<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value / other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::DivAssign<isize> for BoundISize<MIN, MAX> {
	fn div_assign(&mut self, other: isize) {
		self.inner_value = (self.inner_value / other).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Mul for BoundISize<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		return Self::new(self.inner_value * other.inner_value);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Mul<isize> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: isize) -> Self {
		return Self::new(self.inner_value * other);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::MulAssign for BoundISize<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value * other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::MulAssign<isize> for BoundISize<MIN, MAX> {
	fn mul_assign(&mut self, other: isize) {
		self.inner_value = (self.inner_value * other).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Add<i32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn add(self, other: i32) -> Self {
		return Self::new(self.inner_value + other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::AddAssign<i32> for BoundISize<MIN, MAX> {
	fn add_assign(&mut self, other: i32) {
		self.inner_value = (self.inner_value + other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Sub<i32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: i32) -> Self {
		return Self::new(self.inner_value - other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::SubAssign<i32> for BoundISize<MIN, MAX> {
	fn sub_assign(&mut self, other: i32) {
		self.inner_value = (self.inner_value - other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Div<i32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn div(self, other: i32) -> Self {
		return Self::new(self.inner_value / other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::DivAssign<i32> for BoundISize<MIN, MAX> {
	fn div_assign(&mut self, other: i32) {
		self.inner_value = (self.inner_value / other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Mul<i32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: i32) -> Self {
		return Self::new(self.inner_value * other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::MulAssign<i32> for BoundISize<MIN, MAX> {
	fn mul_assign(&mut self, other: i32) {
		self.inner_value = (self.inner_value * other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Add<u32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn add(self, other: u32) -> Self {
		return Self::new(self.inner_value + other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::AddAssign<u32> for BoundISize<MIN, MAX> {
	fn add_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value + other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Sub<u32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: u32) -> Self {
		return Self::new(self.inner_value - other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::SubAssign<u32> for BoundISize<MIN, MAX> {
	fn sub_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value - other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Div<u32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn div(self, other: u32) -> Self {
		return Self::new(self.inner_value / other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::DivAssign<u32> for BoundISize<MIN, MAX> {
	fn div_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value / other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Mul<u32> for BoundISize<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: u32) -> Self {
		return Self::new(self.inner_value * other as isize);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::MulAssign<u32> for BoundISize<MIN, MAX> {
	fn mul_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value * other as isize).clamp(MIN, MAX);
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::Deref for BoundISize<MIN, MAX> {
	type Target = isize;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: isize, const MAX: isize> std::ops::DerefMut for BoundISize<MIN, MAX> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.inner_value;
	}
}

impl<const MIN: isize, const MAX: isize> PartialEq<Self> for BoundISize<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: isize, const MAX: isize> PartialEq<isize> for BoundISize<MIN, MAX> {
	fn eq(&self, other: &isize) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: isize, const MAX: isize> PartialEq<BoundISize<MIN, MAX>> for isize {
	fn eq(&self, other: &BoundISize<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: isize, const MAX: isize> Eq for BoundISize<MIN, MAX> {}

impl<const MIN: isize, const MAX: isize> Hash for BoundISize<MIN, MAX> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner_value.hash(state);
	}
}

impl<const MIN: isize, const MAX: isize> From<isize> for BoundISize<MIN, MAX> {
	fn from(value: isize) -> Self {
		return Self::new(value);
	}
}

impl<const MIN: isize, const MAX: isize> From<i32> for BoundISize<MIN, MAX> {
	fn from(value: i32) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<i64> for BoundISize<MIN, MAX> {
	fn from(value: i64) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<usize> for BoundISize<MIN, MAX> {
	fn from(value: usize) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<u32> for BoundISize<MIN, MAX> {
	fn from(value: u32) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<u64> for BoundISize<MIN, MAX> {
	fn from(value: u64) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<f32> for BoundISize<MIN, MAX> {
	fn from(value: f32) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> From<f64> for BoundISize<MIN, MAX> {
	fn from(value: f64) -> Self {
		return Self::new(value as isize);
	}
}

impl<const MIN: isize, const MAX: isize> Into<i32> for BoundISize<MIN, MAX> {
    fn into(self) -> i32 {
        return self.inner_value as i32;
    }
}

impl<const MIN: isize, const MAX: isize> Into<i64> for BoundISize<MIN, MAX> {
	fn into(self) -> i64 {
		return self.inner_value as i64;
	}
}

impl<const MIN: isize, const MAX: isize> Into<usize> for BoundISize<MIN, MAX> {
	fn into(self) -> usize {
		return if self.inner_value > 0 {
			self.inner_value as usize
		} else {
			0
		};
	}
}

impl<const MIN: isize, const MAX: isize> Into<u32> for BoundISize<MIN, MAX> {
	fn into(self) -> u32 {
		return if self.inner_value > 0 {
			self.inner_value as u32
		} else {
			0
		};
	}
}

impl<const MIN: isize, const MAX: isize> Into<u64> for BoundISize<MIN, MAX> {
	fn into(self) -> u64 {
		return if self.inner_value > 0 {
			self.inner_value as u64
		} else {
			0
		};
	}
}

impl<const MIN: isize, const MAX: isize> Into<f32> for BoundISize<MIN, MAX> {
	fn into(self) -> f32 {
		return self.inner_value as f32;
	}
}

impl<const MIN: isize, const MAX: isize> Into<f64> for BoundISize<MIN, MAX> {
	fn into(self) -> f64 {
		return self.inner_value as f64;
	}
}

#[cfg(test)]
mod tests
{
	use super::BoundISize;

	// Creating a BoundISize instance with a value within the bounds should return the same value when calling get().
	#[test]
	fn test_create_within_bounds() {
		let bound = BoundISize::<-100, 300>::new(50);
		assert_eq!(bound.get(), 50);
	}

	// Adding two BoundISize instances with values within the bounds should return a new instance with the sum of the values within the bounds.
	#[test]
	fn test_add_within_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(50);
		let bound2 = BoundISize::<-100, 300>::new(100);
		let result = bound1 + bound2;
		assert_eq!(result.get(), 150);
	}

	// Subtracting two BoundISize instances with values within the bounds should return a new instance with the difference of the values within the bounds.
	#[test]
	fn test_subtract_within_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(50);
		let bound2 = BoundISize::<-100, 300>::new(100);
		let result = bound1 - bound2;
		assert_eq!(result.get(), -50);
	}

	// Multiplying two BoundISize instances with values within the bounds should return a new instance with the product of the values within the bounds.
	#[test]
	fn test_multiply_within_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(5);
		let bound2 = BoundISize::<-100, 300>::new(10);
		let result = bound1 * bound2;
		assert_eq!(result.get(), 50);
	}

	// Dividing two BoundISize instances with values within the bounds should return a new instance with the quotient of the values within the bounds.
	#[test]
	fn test_divide_within_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(50);
		let bound2 = BoundISize::<-100, 300>::new(10);
		let result = bound1 / bound2;
		assert_eq!(result.get(), 5);
	}

	// Creating a BoundISize instance with a value below the minimum bound should return a new instance with the minimum bound value.
	#[test]
	fn test_create_below_min_bound() {
		let bound = BoundISize::<-100, 300>::new(-200);
		assert_eq!(bound.get(), -100);
	}

	// Creating a BoundISize instance with a value above the maximum bound should return a new instance with the maximum bound value.
	#[test]
	fn test_create_above_max_bound() {
		let bound = BoundISize::<-100, 300>::new(400);
		assert_eq!(bound.get(), 300);
	}

	// Adding two BoundISize instances with values that exceed the bounds should return a new instance with the maximum or minimum bound value, depending on the operation.
	#[test]
	fn test_add_exceed_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(200);
		let bound2 = BoundISize::<-100, 300>::new(200);
		let result = bound1 + bound2;
		assert_eq!(result.get(), 300);
	}

	// Subtracting two BoundISize instances with values that exceed the bounds should return a new instance with the maximum or minimum bound value, depending on the operation.
	#[test]
	fn test_subtract_exceed_bounds() {
		let bound1 = BoundISize::<-100, 300>::new(-200);
		let bound2 = BoundISize::<-100, 300>::new(400);
		let result = bound1 - bound2;
		assert_eq!(result.get(), -100);
	}

	// Multiplying two BoundISize instances with values that exceed the bounds should return a new instance with the maximum or minimum bound value, depending on the operation.
	#[test]
	fn test_multiply_exceed_bounds() {
		let bound1 = BoundISize::<0, 100>::new(50);
		let bound2 = BoundISize::<0, 100>::new(30);
		let result = bound1 * bound2;
		assert_eq!(result.get(), 100);
	}

	// Dividing two BoundISize instances with values that exceed the bounds should return a new instance with the maximum or minimum bound value, depending on the operation.
	#[test]
	fn test_divide_exceed_bounds() {
		let bound1 = BoundISize::<1, 100>::new(50);
		let bound2 = BoundISize::<1, 100>::new(0);
		let result = bound1 / bound2;
		assert_eq!(result.get(), 50);
	}
}
