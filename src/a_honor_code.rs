//! This portion of the exam represents an honor code. By returning `true` from each of these
//! functions you are attesting that you have followed the various rules of the exam.
//!
//! Cheating on this exam will only hurt yourself as you are likely to feel lost and frustrated at
//! the Polkadot Blockchain Academy if you do not have the necessary Rust background to attend.
//!
//! If you are in any doubt of something being allowed or disallowed not, please directly ask the
//! Academy staff for clarification and guidance.

/// You must work independently on this exam. Seeking help from or otherwise working anyone on your
/// solutions while you are completing it is considered cheating. You are encouraged to ask the
/// Academy staff if something is unclear or you are completely stuck.
pub fn exam_done_independently() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

/// The multiple choice portion of the exam must be completed without accessing the internet,
/// books, or any other resources.
pub fn multiple_choice_closed_book() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

/// The multiple choice portion of this exam has several questions asking for the output of a
/// particular programs. It also has questions asking what change to a program would fix a
/// particular issue. For this reason, you are not permitted to compile or run the code snippets
/// provided in the multiple choice portion. You may only trace them manually in your head or on
/// paper. Pretend that this portion of the exam is being administered on paper, and no computer is
/// available at all.
pub fn multiple_choice_no_run() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

/// The coding portion of the exam allows access to books, and websites such as the Rust book, the
/// standard library reference, and others _explicitly listed_ in the exam prompts themselves.
/// However, you are not allowed to look up direct implementation of the specific algorithms we are
/// asking you to write.
///
/// Examples of allowed searches:
/// - How to swap elements of a slice in Rust
/// - What is the bubble sort algorithm
///
/// Examples of searches that are cheating:
/// - Bubble sort implementation in Rust
/// - Bubble sort implementation in C/python/etc
/// - How to {do exact thing the problem asks for} in Rust/C/Python/etc
///
/// If you are in any doubt of something being allowed or disallowed not, please directly ask the
/// Academy staff for clarification and guidance.
pub fn coding_no_copy() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

/// You are not allowed to use external dependencies from `crates.io` or elsewhere unless
/// explicitly stated in the problem.
pub fn coding_no_external_deps() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

/// You are not allowed to use AI assisted coding tools like Github Copilot to complete this exam.
pub fn coding_no_ai_helpers() -> bool {
	// If you have followed this rule, return `true`
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	fn has_honor(f: &dyn Fn() -> bool) {
		assert!(
			f(),
			"Thank you for your honesty in letting us know you have not followed the honor code."
		)
	}

	#[test]
	fn honor_code_respected() {
		has_honor(&multiple_choice_closed_book);
		has_honor(&exam_done_independently);
		has_honor(&multiple_choice_no_run);
		has_honor(&coding_no_copy);
		has_honor(&coding_no_external_deps);
		has_honor(&coding_no_ai_helpers);
	}
}
