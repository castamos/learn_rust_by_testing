/*
 * This file is intended to hold tests that, on purpose, fail to compile;
 * for example to ensure the borrow-checker fails as expected at compile time.
 *
 * Since it's not possible to run a normal `#[test]` that fails to compile, the
 * trick here is to write such tests as documentation examples, which are
 * compiled in sandboxes isolated from the main compilation.
 *
 * If the `compile_fail` attribute is specified in a documentation example,
 * the example passes as a test when it fails to compile.
 *
 * Note that for documentation comments to work, we have to document something.
 * That's why we have functions with empty bodies after the doc comments;
 * such functions are totally irrelevant, and no other actual code is supposed
 * to live in this file.
 */ 

/// The following example fails to compile because `text` is moved into `boxed`,
/// and then we cannot longer access it directly.
///
/// ```compile_fail,E0382
/// fn attempt_to_use_moved_value() {
///     let text = String::from("hi");
///     let boxed = Box::new(text);
///     text.push('!');
/// }
/// ```
fn __doc_test_compile_fail() {
    // Intentionally empty.
}


/// Tests that panic can be included as normal `#[test]`s with the `#[should_panic]`
/// attribute, and that's the recommended way to write them. We are adding one here
/// just to exemplify how to add documentation examples that should panic.
///
/// The following example panics with: `already mutably borrowed: BorrowError`.
/// This is because `inner_string` is mutably borrowing the `String` in the
/// `message_cell`, but then we attempt to borrow it again (immutably, but that
/// doesn't matter) in `inner_string`; thus violating the runtime borrow-checker.
///
/// ```should_panic
/// use std::cell::RefCell;
/// 
/// let message_cell = RefCell::new( String::from("Hello") );
/// assert_eq!(*message_cell.borrow(), "Hello");
///
/// let mut inner_string_mut = message_cell.borrow_mut();
/// (*inner_string_mut).push_str(" world!");
///
/// let inner_string = message_cell.borrow();
/// assert_eq!(*inner_string, "Hello world!");
/// ```
fn __doc_test_should_panic() {
    // Intentionally empty.
}
