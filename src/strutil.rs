#![allow(dead_code)]

/// Check whether a string looks like it might contain embedded JSON.
pub fn looks_like_json(s: &str) -> bool {
	let trimmed = s.trim();
	(trimmed.starts_with('{') && trimmed.ends_with('}')) || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}

/// Normalize consecutive whitespace in a string to single spaces.
pub fn normalize_whitespace(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut prev_space = false;
	for ch in s.chars() {
		if ch.is_whitespace() {
			if !prev_space {
				result.push(' ');
				prev_space = true;
			}
		} else {
			result.push(ch);
			prev_space = false;
		}
	}
	result
}

/// Pad a string on the left to reach a minimum display width.
pub fn pad_left(s: &str, min_width: usize) -> String {
	if s.len() >= min_width {
		s.to_string()
	} else {
		format!("{}{}", " ".repeat(min_width - s.len()), s)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn json_detection() {
		assert!(looks_like_json("{\"key\": 1}"));
		assert!(looks_like_json("[1, 2, 3]"));
		assert!(!looks_like_json("hello world"));
		assert!(!looks_like_json("{incomplete"));
	}

	#[test]
	fn whitespace_normalization() {
		assert_eq!(normalize_whitespace("hello  world"), "hello world");
		assert_eq!(normalize_whitespace("  leading"), " leading");
	}

	#[test]
	fn left_padding() {
		assert_eq!(pad_left("abc", 6), "   abc");
		assert_eq!(pad_left("abcdef", 3), "abcdef");
	}
}
