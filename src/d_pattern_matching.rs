//! Complete the following functions using the pattern matching syntax. That includes the `match`
//! statement of the `matches!()` macro, if you feel like having an "1-liner".
//!
//! You can try and write them imperatively at first as well, but at the end of the day, we want you
//! to write them using the `match` keyword or the `matches!` macro.

/// Returns true if the last two strings in the vector start with `PBA`.
pub fn match_1(input: Vec<String>) -> bool {
	todo!();
}

/// Returns true if the first and last string in the vector start with `PBA`.
pub fn match_2(input: Vec<String>) -> bool {
	todo!();
}

/// Returns true if the first item in `input` is true.
pub fn match_3(input: (bool, bool, bool)) -> bool {
	todo!();
}

/// Returns true if the input is `Ok(x)` of some even `x`.
pub fn match_4(input: Result<u32, &'static str>) -> bool {
	todo!();
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	todo!()
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_match_1_true() {
		let strs = vec![
			"Hello".to_string(),
			"World".to_string(),
			"PBA".to_string(),
			"PBASDFGR".to_string(),
		];
		assert!(match_1(strs))
	}

	#[test]
	fn test_match_1_false() {
		let strs = vec!["Hello".to_string(), "World".to_string(), "PBA".to_string()];
		assert!(!match_1(strs))
	}

	#[test]
	fn test_match_2() {
		let strs = vec![
			"PBAHello".to_string(),
			"World".to_string(),
			"PBA".to_string(),
			"PBASDFGR".to_string(),
		];
		assert!(match_2(strs))
	}

	#[test]
	fn test_match_3() {
		assert!(match_3((true, false, true)))
	}

	#[test]
	fn test_match_4_true() {
		assert!(match_4(Ok(6)))
	}

	#[test]
	fn test_match_4_false() {
		assert!(!match_4(Err("bogus")))
	}
}
