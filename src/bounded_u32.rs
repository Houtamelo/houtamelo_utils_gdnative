use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct BoundU32<const MIN: u32, const MAX: u32> { inner_value: u32 }

impl<const MIN: u32, const MAX: u32> BoundU32<MIN, MAX> {
	pub fn new(inner_value: u32) -> Self {
		return Self { inner_value: inner_value.clamp(MIN, MAX) };
	}

	pub fn get(&self) -> u32 {
		return self.inner_value;
	}

	pub fn set(&mut self, value: u32) {
		self.inner_value = value;
	}
}

impl<const MIN: u32, const MAX: u32> Default for BoundU32<MIN, MAX> {
	fn default() -> Self {
		return Self::new(MIN);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Add for BoundU32<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		return Self::new(self.inner_value + other.inner_value);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Add<u32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn add(self, other: u32) -> Self {
		return Self::new(self.inner_value + other);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Add<isize> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn add(self, other: isize) -> Self {
		let mut result = self.inner_value as isize + other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::AddAssign for BoundU32<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value + other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::AddAssign<u32> for BoundU32<MIN, MAX> {
	fn add_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value + other).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::AddAssign<isize> for BoundU32<MIN, MAX> {
	fn add_assign(&mut self, other: isize) {
		let mut result = self.inner_value as isize + other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Sub for BoundU32<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		return Self::new(self.inner_value - other.inner_value);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Sub<u32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: u32) -> Self {
		return Self::new(self.inner_value - other);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Sub<isize> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: isize) -> Self {
		let mut result = self.inner_value as isize - other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::SubAssign for BoundU32<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value - other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::SubAssign<u32> for BoundU32<MIN, MAX> {
	fn sub_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value - other).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::SubAssign<isize> for BoundU32<MIN, MAX> {
	fn sub_assign(&mut self, other: isize) {
		let mut result = self.inner_value as isize - other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Div for BoundU32<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		return Self::new(self.inner_value / other.inner_value);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Div<u32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn div(self, other: u32) -> Self {
		return Self::new(self.inner_value / other);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Div<isize> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn div(self, other: isize) -> Self {
		let mut result = self.inner_value as isize / other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DivAssign for BoundU32<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value / other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DivAssign<u32> for BoundU32<MIN, MAX> {
	fn div_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value / other).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DivAssign<isize> for BoundU32<MIN, MAX> {
	fn div_assign(&mut self, other: isize) {
		let mut result = self.inner_value as isize / other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Mul for BoundU32<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		return Self::new(self.inner_value * other.inner_value);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Mul<u32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: u32) -> Self {
		return Self::new(self.inner_value * other);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Mul<isize> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: isize) -> Self {
		let mut result = self.inner_value as isize * other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::MulAssign for BoundU32<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.inner_value = (self.inner_value * other.inner_value).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::MulAssign<u32> for BoundU32<MIN, MAX> {
	fn mul_assign(&mut self, other: u32) {
		self.inner_value = (self.inner_value * other).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::MulAssign<isize> for BoundU32<MIN, MAX> {
	fn mul_assign(&mut self, other: isize) {
		let mut result = self.inner_value as isize * other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Deref for BoundU32<MIN, MAX> {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DerefMut for BoundU32<MIN, MAX> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.inner_value;
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<Self> for BoundU32<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<u32> for BoundU32<MIN, MAX> {
	fn eq(&self, other: &u32) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<isize> for BoundU32<MIN, MAX> {
	fn eq(&self, other: &isize) -> bool {
		if *other < 0 {
			return false;
		}

		return self.inner_value == *other as u32;
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<BoundU32<MIN, MAX>> for u32 {
	fn eq(&self, other: &BoundU32<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: u32, const MAX: u32> Eq for BoundU32<MIN, MAX> {}

impl<const MIN: u32, const MAX: u32> PartialOrd<Self> for BoundU32<MIN, MAX> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		return self.inner_value.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: u32, const MAX: u32> PartialOrd<u32> for BoundU32<MIN, MAX> {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		return self.inner_value.partial_cmp(other);
	}
}

impl<const MIN: u32, const MAX: u32> PartialOrd<isize> for BoundU32<MIN, MAX> {
	fn partial_cmp(&self, other: &isize) -> Option<std::cmp::Ordering> {
		if *other < 0 {
			return Some(std::cmp::Ordering::Less);
		}

		return self.inner_value.partial_cmp(&(*other as u32));
	}
}

impl<const MIN: u32, const MAX: u32> From<u32> for BoundU32<MIN, MAX> {
	fn from(value: u32) -> Self {
		return Self::new(value);
	}
}

impl<const MIN: u32, const MAX: u32> From<usize> for BoundU32<MIN, MAX> {
	fn from(value: usize) -> Self {
		return Self::new(value as u32);
	}
}

impl<const MIN: u32, const MAX: u32> From<isize> for BoundU32<MIN, MAX> {
	fn from(value: isize) -> Self {
		if value < 0 {
			return Self::new(0);
		} else {
			return Self::new(value as u32);
		}
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Add<i32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn add(self, other: i32) -> Self {
		let mut result = self.inner_value as i32 + other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::AddAssign<i32> for BoundU32<MIN, MAX> {
	fn add_assign(&mut self, other: i32) {
		let mut result = self.inner_value as i32 + other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Sub<i32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: i32) -> Self {
		let mut result = self.inner_value as i32 - other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::SubAssign<i32> for BoundU32<MIN, MAX> {
	fn sub_assign(&mut self, other: i32) {
		let mut result = self.inner_value as i32 - other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Div<i32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn div(self, other: i32) -> Self {
		let mut result = self.inner_value as i32 / other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DivAssign<i32> for BoundU32<MIN, MAX> {
	fn div_assign(&mut self, other: i32) {
		let mut result = self.inner_value as i32 / other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Mul<i32> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: i32) -> Self {
		let mut result = self.inner_value as i32 * other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::MulAssign<i32> for BoundU32<MIN, MAX> {
	fn mul_assign(&mut self, other: i32) {
		let mut result = self.inner_value as i32 * other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<i32> for BoundU32<MIN, MAX> {
	fn eq(&self, other: &i32) -> bool {
		if *other < 0 {
			return false;
		}

		return self.inner_value == *other as u32;
	}
}

impl<const MIN: u32, const MAX: u32> PartialOrd<i32> for BoundU32<MIN, MAX> {
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		if *other < 0 {
			return Some(std::cmp::Ordering::Less);
		}

		return self.inner_value.partial_cmp(&(*other as u32));
	}
}

impl<const MIN: u32, const MAX: u32> From<i32> for BoundU32<MIN, MAX> {
	fn from(value: i32) -> Self {
		if value < 0 {
			return Self::new(0);
		} else {
			return Self::new(value as u32);
		}
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Add<i64> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn add(self, other: i64) -> Self {
		let mut result = self.inner_value as i64 + other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::AddAssign<i64> for BoundU32<MIN, MAX> {
	fn add_assign(&mut self, other: i64) {
		let mut result = self.inner_value as i64 + other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Sub<i64> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: i64) -> Self {
		let mut result = self.inner_value as i64 - other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::SubAssign<i64> for BoundU32<MIN, MAX> {
	fn sub_assign(&mut self, other: i64) {
		let mut result = self.inner_value as i64 - other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Div<i64> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn div(self, other: i64) -> Self {
		let mut result = self.inner_value as i64 / other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::DivAssign<i64> for BoundU32<MIN, MAX> {
	fn div_assign(&mut self, other: i64) {
		let mut result = self.inner_value as i64 / other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::Mul<i64> for BoundU32<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: i64) -> Self {
		let mut result = self.inner_value as i64 * other;
		if result < 0 {
			result = 0;
		}

		return Self::new(result as u32);
	}
}

impl<const MIN: u32, const MAX: u32> std::ops::MulAssign<i64> for BoundU32<MIN, MAX> {
	fn mul_assign(&mut self, other: i64) {
		let mut result = self.inner_value as i64 * other;
		if result < 0 {
			result = 0;
		}

		self.inner_value = (result as u32).clamp(MIN, MAX);
	}
}

impl<const MIN: u32, const MAX: u32> PartialEq<i64> for BoundU32<MIN, MAX> {
	fn eq(&self, other: &i64) -> bool {
		if *other < 0 {
			return false;
		}

		return self.inner_value == *other as u32;
	}
}

impl<const MIN: u32, const MAX: u32> PartialOrd<i64> for BoundU32<MIN, MAX> {
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		if *other < 0 {
			return Some(std::cmp::Ordering::Less);
		}

		return self.inner_value.partial_cmp(&(*other as u32));
	}
}

impl<const MIN: u32, const MAX: u32> Hash for BoundU32<MIN, MAX> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner_value.hash(state);
	}
}

impl<const MIN: u32, const MAX: u32> From<i64> for BoundU32<MIN, MAX> {
	fn from(value: i64) -> Self {
		if value < 0 {
			return Self::new(0);
		} else {
			return Self::new(value as u32);
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::BoundU32;

	// Creating a new BoundU32 instance with a valid inner value should return the instance with the same inner value.
	#[test]
	fn test_new_instance_with_valid_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		assert_eq!(bound_u32.get(), 50);
	}

	// Adding two BoundU32 instances should return a new BoundU32 instance with the sum of their inner values.
	#[test]
	fn test_add_two_instances() {
		let bound_u32_1 = BoundU32::<20, 300>::new(50);
		let bound_u32_2 = BoundU32::<20, 300>::new(100);
		let result = bound_u32_1 + bound_u32_2;
		assert_eq!(result.get(), 150);
	}

	// Adding a BoundU32 instance and a u32 value should return a new BoundU32 instance with the sum of their inner values.
	#[test]
	fn test_add_instance_and_u32_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		let result: BoundU32<20, 300> = bound_u32 + 100;
		assert_eq!(result.get(), 150);
	}

	// Creating a new BoundU32 instance with the minimum allowed inner value should return the instance with the same inner value.
	#[test]
	fn test_new_instance_with_minimum_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(20);
		assert_eq!(bound_u32.get(), 20);
	}

	// Creating a new BoundU32 instance with the maximum allowed inner value should return the instance with the same inner value.
	#[test]
	fn test_new_instance_with_maximum_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(300);
		assert_eq!(bound_u32.get(), 300);
	}

	// Adding two BoundU32 instances with the maximum allowed inner values should return a new BoundU32 instance with the sum of their inner values, clamped to the range [MIN, MAX].
	#[test]
	fn test_add_two_instances_with_maximum_inner_values() {
		let bound_u32_1 = BoundU32::<20, 300>::new(300);
		let bound_u32_2 = BoundU32::<20, 300>::new(200);
		let result = bound_u32_1 + bound_u32_2;
		assert_eq!(result.get(), 300);
	}

	// Creating a new BoundU32 instance with a valid inner value should return the instance with the same inner value.
	#[test]
	fn create_new_instance_with_valid_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		assert_eq!(bound_u32.get(), 50);
	}

	// Adding two BoundU32 instances should return a new BoundU32 instance with the sum of their inner values.
	#[test]
	fn add_two_instances() {
		let bound_u32_1 = BoundU32::<20, 300>::new(50);
		let bound_u32_2 = BoundU32::<20, 300>::new(100);
		let result = bound_u32_1 + bound_u32_2;
		assert_eq!(result.get(), 150);
	}

	// Adding a BoundU32 instance and a u32 value should return a new BoundU32 instance with the sum of their inner values.
	#[test]
	fn add_instance_and_u32_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		let result: BoundU32<20, 300> = bound_u32 + 100;
		assert_eq!(result.get(), 150);
	}

	// Adding a BoundU32 instance and an isize value should return a new BoundU32 instance with the sum of their inner values, clamped to the range [0, MAX].
	#[test]
	fn add_instance_and_isize_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		let result = bound_u32 + 100isize;
		assert_eq!(result.get(), 150);
	}

	// Adding a BoundU32 instance and an i32 value should return a new BoundU32 instance with the sum of their inner values, clamped to the range [0, MAX].
	#[test]
	fn add_instance_and_i32_value() {
		let bound_u32 = BoundU32::<20, 300>::new(50);
		let result = bound_u32 + 100i32;
		assert_eq!(result.get(), 150);
	}

	// Creating a new BoundU32 instance with the minimum allowed inner value should return the instance with the same inner value.
	#[test]
	fn create_new_instance_with_minimum_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(20);
		assert_eq!(bound_u32.get(), 20);
	}

	// Creating a new BoundU32 instance with the maximum allowed inner value should return the instance with the same inner value.
	#[test]
	fn create_new_instance_with_maximum_inner_value() {
		let bound_u32 = BoundU32::<20, 300>::new(300);
		assert_eq!(bound_u32.get(), 300);
	}

	// Adding two BoundU32 instances with the maximum allowed inner values should return a new BoundU32 instance with the sum of their inner values, clamped to the range [MIN, MAX].
	#[test]
	fn add_two_instances_with_maximum_inner_values() {
		let bound_u32_1 = BoundU32::<20, 300>::new(300);
		let bound_u32_2 = BoundU32::<20, 300>::new(200);
		let result = bound_u32_1 + bound_u32_2;
		assert_eq!(result.get(), 300);
	}

	// Adding a BoundU32 instance and a u32 value that would result in an overflow should return a new BoundU32 instance with the maximum allowed inner value.
	#[test]
	fn add_instance_and_overflowing_u32_value() {
		let bound_u32 = BoundU32::<20, 300>::new(250);
		let result = bound_u32 + 100u32;
		assert_eq!(result.get(), 300);
	}

	// Adding a BoundU32 instance and an isize value that would result in an overflow should return a new BoundU32 instance with the maximum allowed inner value.
	#[test]
	fn add_instance_and_overflowing_isize_value() {
		let bound_u32 = BoundU32::<20, 300>::new(250);
		let result = bound_u32 + 100000isize;
		assert_eq!(result.get(), 300);
	}

	#[test]
	fn add_deref() {
		let mut bound_u32 = BoundU32::<20, 300>::new(250);
		let mut_ref = &mut bound_u32;
		*mut_ref += 100;
		assert_eq!(bound_u32.get(), 300);
	}
}
