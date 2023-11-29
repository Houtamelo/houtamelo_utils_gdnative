#[macro_export]
macro_rules! any_matches {
    ($collection: ident, $pattern: pat) => {
		$collection.iter().any(|d| matches!(d, $pattern))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_any() {
		let v = vec![1, 2, 3];
		let result = any_matches!(v, 2);
		assert!(result);
	}
}
