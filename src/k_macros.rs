//! Here we test your ability to write macros.
//!
//! Make sure these macros are importable from the root of your crate, and usable in an external
//! crate.

// A common Rust macro is `vec![...]` used for creating vectors of literal values. Without this
// macro, one would need to create an empty vector, and push each item to it individually.
//
// Your task here is to create an analogous macro for creating hashmaps pre-populated with literal
// values. The macro should be called like follows:
//
// let map1: HashMap<u32, u32> = map![1 =>2, 3 => 4, 5 => 6];
#[macro_export]
macro_rules! map {
	( $($todo:tt)* ) => {
		Default::default()
	};
}

/// Next, write a macro that implements a `get` function on a type.
///
/// Observe the following `trait Get`. It is merely a way to convey a value of type `T` via a `type`
/// (ie) a struct that implements `Get`. An example of manually building one of such structs is what
/// you see in `struct Seven`. It works, but it is verbose. We want a macro that would automatically
/// generate this implementation for us, as such:
///
/// ```
/// use pba_entrance_exam::impl_get;
/// impl_get! {
/// 	// implements `Get<u32>` for `struct Six`
/// 	Six: u32 = 6;
/// 	// implements `Get<u16>` for `struct FortyTwo`
/// 	FortyTwo: u16 = 42;
/// }
/// ```
///
/// For now, your macro could only support literals as the right hand side of `=`. But you can think
/// about the ramifications of allowing arbitrary expressions as well.
///
/// An important detail that you need to be aware of is that this macro could be called from outside
/// of this file as well. This means that you need to reference `trait Get` with its full path,
/// using `$crate`. Read more: `https://doc.rust-lang.org/reference/macros-by-example.html#hygiene`

pub trait Get<T> {
	fn get() -> T;
}

struct Seven;
impl Get<u32> for Seven {
	fn get() -> u32 {
		7
	}
}

// note that you should first-thing change `$($todo:tt)*`, this simply means 'accept anything'.

#[macro_export]
macro_rules! impl_get {
	( $($todo:tt)* ) => {};
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
	use std::collections::HashMap;

	#[test]
	fn map() {
		let macro_generated: HashMap<u32, u32> = map![1 => 2, 3 => 4, 5 => 6];
		let mut expected = HashMap::<u32, u32>::new();
		expected.insert(1, 2);
		expected.insert(3, 4);
		expected.insert(5, 6);

		assert_eq!(macro_generated, expected);
	}

	#[test]
	fn impl_get() {
		impl_get!(
			// should generate `struct Foo` that implements `Get<u32>`
			Foo: u32 = 10;
			// should generate `pub struct Bar` that implements `Get<u32>`
			pub Bar: u32 = 42;
			// should generate `pub struct Baz` that has implements `Get<u16>`.
			pub Baz: u16 = 21;
		);

		// you should be able to make these work.
		// assert_eq!(Foo::get(), 10);
		// assert_eq!(Bar::get(), 42);
		// assert_eq!(Baz::get(), 21);

		// As an extra, ungraded, challenge, try to make this work.
		// This is not part of the main problem because it requires the nightly compiler.
		// const CONST: u16 = Baz::get();
		// assert_eq!(CONST, 21);
	}
}
