

/// ```compile_fail,E0382
/// fn attempt_to_use_moved_value() {
///     let text = String::from("hi");
///     let boxed = Box::new(text);
///     text.push('!');
/// }
/// ```
fn __doc_test() {
    // Intentionally empty.
}
