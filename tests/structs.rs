
/// Tests related to `struct`s.
///
#[cfg(test)]
mod test_structs {

    #[test]
    fn tuple_struct_as_type_alias() {
        struct MyInt(i32);
        let my_int = MyInt(25);
        assert_eq!(my_int.0, 25, "One-element tuple-struct accessed via `.0` syntax.");
    }
}

