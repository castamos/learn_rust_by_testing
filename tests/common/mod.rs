/// Generic test utilities.
/// 
/// Other way to have this is as a shared crate listed as dev-dependency.
/// That allows, for example, running examples and unit tests for the test
/// utilities themselves.
///

use std::any::type_name;


/// Returns the type of the given parameter, as a string slice.
///
///
/// # Examples
///
/// assert_eq!(type_name(5), "i32");
///
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
