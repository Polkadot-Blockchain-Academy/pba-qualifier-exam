//! # Multiple Choice Questions
//!
//! **This portion of the exam is to be completed without accessing the internet, books, the rust
//! compiler, or any other resources.**
//!
//! There are several questions asking for the output of particular programs or asking what change
//! would fix a particular issue. For this reason, you are **not permitted to compile or run the
//! code snippets**. You may only trace them manually in your head or on paper. Pretend that this
//! portion of the exam is being administered on paper, and no computer is available at all.
//!
//! To enable us to auto-grade your answer, you implement the code that returns the `char` representing
//! your answer.
//!
//! If you would like a rendered and styled version of these questions simply build an open the Rust
//! Docs for this module, and navigate the the multiple choice question module docs by the command:
//!
//! ```sh
//! cargo doc --open
//! ```

/// ## Question 1
///
/// Consider this Rust code when answering question 1:
///
/// ```notest
/// let x = 4;
/// let y = 5;
/// let z = x - y;
/// let y = 3;
/// ```
///
/// ### Question 1_A
///
/// What type is the variable x?
///
/// - a) usize
/// - b) isize
/// - c) u32
/// - d) i32
/// - e) int
pub fn answer_1_a() -> char {
	todo!()
}

/// ### Question 1_B
///
/// Why can the variable `y` be reassigned in the last line when it was not declared as `mut`?
///
/// - a) Mutable variables are the default.
/// - b) Even immutable variables can be re-assigned, only `const`s cannot.
/// - c) It is not reassigned, it is shadowed.
/// - d) Because the original `y` value was moved into z` on line 3.
/// - e) Because it is being borrowed.
pub fn answer_1_b() -> char {
	todo!()
}

/// ### Question 1_C
///
/// What is the value of `z` at the end of the program?
///
/// - a) 1
/// - b) -1
/// - c) 4294967295 (u32 max value)
/// - d) 0
/// - e) 511
pub fn answer_1_c() -> char {
	todo!()
}

/// ### Question 1_D
///
/// Which of the following is not a valid looping construct in Rust?
///
/// - a) `do {} while x > 0`
/// - b) `for _ in 1..2 {}`
/// - c) `while x > 0 {}`
/// - d) `loop {}`
/// - e) `for _ in vec![1, 2, 3] {}`
pub fn answer_1_d() -> char {
	todo!()
}

/// ## Question 2
///
/// Consider this Rust code:
///
/// ```notest
/// /// Check whether a value is between 5 and 10, and return a user-readable
/// /// string explaining the findings
/// fn value_in_range(n: u8) -> String {
/// 	if n < 5 {
/// 		let result = String::from("value is too small");
/// 	}
/// 	else if n > 10 {
/// 		let result = String::from("value is too large");
/// 	}
/// 	else {
/// 		let result = String::from("value is just right");
/// 	}
/// 	result
/// }
/// ```
///
/// Why does the above program fail to compile?
///
/// - a) It needs a lifetime parameter.
/// - b) Rust does not support the `else if` syntax.
/// - c) The `u8` type is not appropriate here.
/// - d) It has the wrong return type.
/// - e) The variable `result` is out of scope by the end.
pub fn answer_2() -> char {
	todo!()
}

/// ## Question 3
///
/// Consider this Rust code when answering question 3:
///
/// ```notest
/// fn mystery(n: u32) -> Vec<u32> {
/// 	let mut seq = Vec::new();
/// 	for k in 1..n {
/// 		seq.push(k * k + 3);
/// 	}
/// 	seq
/// }
/// ```
///
/// ### Question 3_A
///
/// What value is returned when calling `mystery(6)`?
///
/// - a) [4, 7, 12, 19, 28]
/// - b) [3, 4, 7, 12, 19]
/// - c) [4, 7, 12, 19, 28, 39]
/// - d) [4, 10, 18, 28, 40]
/// - e) [28, 19, 12, 7, 4]
pub fn answer_3_a() -> char {
	todo!()
}

/// ### Question 3_B
///
/// Consider changing the range `1..n`. Which of the following would _not_ cause the loop to execute
/// one additional time:
///
/// - a) `1..=n`
/// - b) `0..n`
/// - c) `1..n+1`
/// - d) `(1..n).step_by(1)`
/// - e) `(1..=n).step_by(1)`
pub fn answer_3_b() -> char {
	todo!()
}

/// ## Question 4
///
/// Consider this Rust code when answering question 4:
///
/// ```notest
/// fn sort(items: &mut [u32]);
/// ```
///
/// ### Question 4_A
///
/// The signature above makes sense for a sorting algorithm that:
///
/// - a) Returns a copy of the data in a new Vec in sorted order.
/// - b) Returns a copy of the data in a new slice in sorted order.
/// - c) Returns a new subslice of the same data in sorted order
/// - d) Sorts the data "in place" by moving elements around.
/// - e) Sorts the data and removes duplicate items.
pub fn answer_4_a() -> char {
	todo!()
}

/// ### Question 4_B
///
/// How would this signature need to change to sort arbitrary data?
///
/// - a) `fn sort<T>(items: &mut [T])`
/// - b) `fn sort<T: Ord>(items: &mut [T]);`
/// - c) `fn sort<T: PartialOrd>(items: &mut [T]);`
/// - d) `fn sort(items: &mut [T: PartialOrd]);`
/// - e) `fn sort(items: &mut [T: Ord]);`
pub fn answer_4_b() -> char {
	todo!()
}

/// ## Question 5
///
/// Which of the following tools is used to install Rust and manage Rust versions and components?
///
/// - a) cargo
/// - b) rustc
/// - c) rustup
/// - d) clippy
/// - e) nvm
pub fn answer_5() -> char {
	todo!()
}

/// ## Question 6
///
/// Consider this Rust code:
///
/// ```notest
/// struct Fraction {
/// 	numerator: u32,
/// 	denominator: u32,
/// }
///
/// impl From<u32> for Fraction {
/// 	fn from(n: u32) -> Self {
/// 		Self {
/// 			numerator: n,
/// 			denominator: 1,
/// 		}
/// 	}
/// }
///
/// fn main() {
/// 	let a: u32 = 5;
/// 	let b: Fraction = a.into();
/// }
/// ```
///
/// Does the above code compile? Why or why not?
///
/// - a) Yes, because in mathematics, any integer can be turned into a fraction.
/// - b) No, because the denominator should be `n` and the numerator `1`.
/// - c) No, because the `.into()` method comes from the `Into` trait which is not implemented.
/// - d) Yes, because the implementation of `From` implies an implementation of `Into`.
/// - e) Yes, because the implementation of `Into` implies an implementation of `From`.
pub fn answer_6() -> char {
	todo!()
}

/// ## Question 7
///
/// Consider this Rust code:
///
/// ```notest
/// use std::num::ParseIntError;
///
/// enum OutOfRangeError {
/// 	TooLarge,
/// 	TooSmall,
/// 	NotEvenANumber,
/// }
///
/// impl From<ParseIntError> for OutOfRangeError {
/// 	fn from(_e: ParseIntError) -> Self {
/// 		Self::NotEvenANumber
/// 	}
/// }
///
/// fn string_to_int_in_range(s: String) -> Result<u32, OutOfRangeError> {
/// 	// Given: The u32::from_str_radix function returns Result<u32, ParseIntError>
/// 	let n: u32 = u32::from_str_radix(&s, 10)?;
///
/// 	match n {
/// 		n if n < 5 => Err(OutOfRangeError::TooSmall),
/// 		n if n > 100 => Err(OutOfRangeError::TooLarge),
/// 		n => Ok(n),
/// 	}
/// }
/// ```
///
/// Does this code compile? Why or why not?
///
/// - a) No, because you cannot use `if` as part of a pattern.
/// - b) No, because `from_str_radix` does not return `Err(OutOfRangeError)` in any case, and that
///   is the error type required by `string_to_int_in_range`.
/// - c) Yes, because any error type can be converted to any other error type by the `?` operator.
/// - d) No, because the potential `ParseIntError` is never handled.
/// - e) Yes, because the `?` operator implicitly performs a `.into()` before returning the error.
pub fn answer_7() -> char {
	todo!()
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

	fn sanity_check(f: &dyn Fn() -> char) {
		assert!(
			"abcde".contains(f()),
			"{}",
			"You have not returned an answer a, b, c, d, or e."
		)
	}

	#[test]
	fn answer_1_a_sanity_check() {
		sanity_check(&answer_1_a)
	}

	#[test]
	fn answer_1_b_sanity_check() {
		sanity_check(&answer_1_b)
	}

	#[test]
	fn answer_1_c_sanity_check() {
		sanity_check(&answer_1_c)
	}

	#[test]
	fn answer_1_d_sanity_check() {
		sanity_check(&answer_1_d)
	}

	#[test]
	fn answer_2_sanity_check() {
		sanity_check(&answer_2)
	}

	#[test]
	fn answer_3_a_sanity_check() {
		sanity_check(&answer_3_a)
	}

	#[test]
	fn answer_3_b_sanity_check() {
		sanity_check(&answer_3_b)
	}

	#[test]
	fn answer_4_a_sanity_check() {
		sanity_check(&answer_4_a)
	}

	#[test]
	fn answer_4_b_sanity_check() {
		sanity_check(&answer_4_b)
	}

	#[test]
	fn answer_5_sanity_check() {
		sanity_check(&answer_5)
	}

	#[test]
	fn answer_6_sanity_check() {
		sanity_check(&answer_6)
	}

	#[test]
	fn answer_7_sanity_check() {
		sanity_check(&answer_7)
	}
}
