

//#[macro_use]
//extern crate static_assertions;

mod common;

/// Tests to verify the default type inferred when there are multiple
/// valid possibilities.
///
#[cfg(test)]
mod test_basic_type_inference {
    use super::common::type_of;

    #[test] fn integer() { assert_eq!(type_of(3),    "i32"); }
    #[test] fn float()   { assert_eq!(type_of(3.14), "f64"); }

}
