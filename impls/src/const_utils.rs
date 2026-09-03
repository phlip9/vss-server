/// Concatenates constant string expressions into a single constant string.
macro_rules! const_concat_str {
	($($string:expr),* $(,)?) => {{
		$(const _: &str = $string;)*
		const LEN: usize = 0 $(+ $string.len())*;
		const BYTES: [u8; LEN] =
			$crate::const_utils::const_concat_str_inner::<LEN>(&[$($string.as_bytes()),*]);
		match std::str::from_utf8(&BYTES) {
			Ok(string) => string,
			Err(_) => panic!("concatenated string was not valid UTF-8"),
		}
	}};
}

pub(crate) use const_concat_str;

/// Copies constant byte slices into a single constant byte array.
pub(crate) const fn const_concat_str_inner<const LEN: usize>(slices: &[&[u8]]) -> [u8; LEN] {
	let mut bytes = [0; LEN];
	let mut base = 0;
	let mut i = 0;
	while i < slices.len() {
		let slice = slices[i];
		let mut j = 0;
		while j < slice.len() {
			bytes[base + j] = slice[j];
			j += 1;
		}
		base += slice.len();
		i += 1;
	}
	assert!(base == LEN, "invalid concatenated string length");
	bytes
}

#[cfg(test)]
mod tests {
	use super::const_concat_str;

	#[test]
	fn concatenates_const_string_expressions() {
		const NAME: &str = const_concat_str!("HUGE", " ", "MAN");
		const GREETING: &str = const_concat_str!("Hello ", NAME, "!!");
		assert_eq!(GREETING, "Hello HUGE MAN!!");
	}
}
