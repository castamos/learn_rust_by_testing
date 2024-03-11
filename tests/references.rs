
//mod common;

/// Tests related to references and smart pointers.
///
#[cfg(test)]
mod test_references {

    //use super::common::type_of;

    #[test]
    fn dereference_reference() {
        let num = 5;
        let num_ref = &num;

        assert_eq!(num, 5, "Access value directly");
        assert_eq!(*num_ref, 5, "Access referenced value by using the dereference operator: `*`");
    }


    #[test]
    fn dereference_box() {
        let num = 5;
        let num_box = Box::new(num);

        assert_eq!(num, 5, "Access value directly");
        assert_eq!(*num_box, 5, "Access boxed value by using the dereference operator: `*`");
    }


    /// Verifies the underlying value pointed to by a reference can be modified through the
    /// reference.
    #[test]
    fn modify_value_through_reference() {
        let mut num = 5;
        let num_ref = &mut num;

        assert_eq!(*num_ref, 5, "Access referenced value by using the dereference operator: `*`");
        *num_ref = 6;
        assert_eq!(num, 6, "Referenced value modified throw reference.");
    }


    /// Verifies that the integer value contained in a `Box` is a copy of the value passed
    /// to `Box::new()`; then as such, changing the boxed value does not modify the original value
    /// passed to the `Box`'s constructor.
    #[test]
    fn modify_boxed_value() {
        let num = 5;
        let mut num_box = Box::new(num);

        assert_eq!(*num_box, 5, "Access referenced value by using the dereference operator: `*`");

        *num_box = 6;
        assert_eq!(*num_box, 6, "Boxed value modfied.");
        assert_eq!(num,      5, "Original value unchanged (boxed is a copy).");
    }


    /// Verifies a boxed mutable reference can alter the original value by double dereference.
    #[test]
    fn modify_boxed_value_by_reference() {
        let mut num = 5;
        let num_box_mut = Box::new(&mut num);

        assert_eq!(**num_box_mut, 5, "Access referenced value by double dereference: `**`");

        **num_box_mut = 7;
        assert_eq!(**num_box_mut, 7, "Boxed value modfied.");
        assert_eq!(num,           7, "Original value changed since passed as mutable reference.");
    }


    /// Verifies that values not implementing the `Copy` trait are moved into the box.
    #[test]
    fn move_non_copy_value_to_box() {
        let text = String::from("Hello world");
        let mut text_box = Box::new(text);

        assert_eq!(*text_box, "Hello world");
        (*text_box).push('!');
        assert_eq!(*text_box, "Hello world!");
        text_box.push('!');
        assert_eq!(*text_box, "Hello world!!");
    }
}


/// Tests for user-defined dereferenceable types ("smart pointers").
/// Rust Book section 15.2.
#[cfg(test)]
mod test_user_defined_deref {

    use std::ops::{Deref,DerefMut}; // Traits for dereferenceable types.

    /// A single-element tuple acting as a trivial generic container.
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        /// Trivial constructor.
        fn new(data: T) -> MyBox<T> {
            MyBox(data)
        }
    }


    /// Implementation of the standarad `Deref` trait for getting immutable references.
    impl<T> Deref for MyBox<T> {

        // `Deref` requires us to define a type named `Target`, which is the underlying type
        // our pointer points to.
        type Target = T;

        /// Immutable dereference. Returns a `&Target` reference to the data contained in our box.
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }


    /// Implementation of the standarad `DerefMut` trait for getting mutable references.
    impl<T> DerefMut for MyBox<T> {

        /// Mutable dereference. Returns a `&mut Target` reference to the data contained in our
        /// box.
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }


    #[test]
    fn dereference_user_defined_pointer() {
        let mybox = MyBox::new(5);
        assert_eq!(*mybox, 5, "User defined pointer dereferenced using the `*` operator");
        assert_eq!(*mybox, *(mybox.deref()), "Equivalent dereference expressions");
    }

    #[test]
    fn immutable_reference_explicit_coercion() {
        let mybox = MyBox::new( String::from("Hi") );

        let immut_ref = &mybox as &String;
        //               ^^^^^^^^^^ Explicit cast from `&MyBox` to `&String`.
        
        assert_eq!(immut_ref, "Hi", "Compare via the casted `&String`");
        assert_eq!(*immut_ref, "Hi", "Compare via the dereferenced `String`");

        // This fails to compile:
        //assert_eq!(&mybox, "Hi");
        // But this works:
        assert_eq!(&mybox as &String, "Hi", "Compare with explicit cast");
        assert_eq!(*mybox , "Hi", "Compare by dereference to `String`");
    }

    #[test]
    fn immutable_reference_implicit_coercion() {

        let mybox = MyBox::new( String::from("Hi") );
        let immut_ref: &String = &mybox;
        assert_eq!(immut_ref, "Hi", "Compare via the casted implicitly casted `&String`");
    }

    #[test]
    fn mutable_reference_explicit_coercion() {

        let mut mybox = MyBox::new( String::from("Hola mundo!") );
        //  ^^^ `mybox` needs to be mutable in order to get mutable references to the contained
        //  data.
        
        let mut_ref = &mut mybox as &mut String;
        //                       ^^^^^^^^^^^^^^ Explicit cast from `&mut MyBox` to `&mut String`.
        mut_ref.push('!');

        assert_eq!(*mybox, "Hola mundo!!", "Underlying value modified through mutable reference");
    }

    #[test]
    fn mutable_reference_implicit_coercion() {

        let mut mybox = MyBox::new( String::from("Hola mundo!") );

        // `&mut MyBox` gets coerced implicitly to `&mut String` due to type inference:
        let mut_ref : &mut String = &mut mybox; 
        //            ^^^^          ^^^^
        // Reference anotations always need to be provided explicitly, type inference does not
        // resolve this.
 
        mut_ref.push('!');

        assert_eq!(*mybox, "Hola mundo!!", "Underlying value modified through mutable reference");
    }

    #[test]
    fn method_call_implicit_coercion_() {
        let mut mybox = MyBox::new( String::from("Hello") );

        // This works because the `push` method expects `mybox` to be of type `&mut Self`,
        // and we have an appropriate dereference to that type:
        mybox.push('!');

        assert_eq!(*mybox, "Hello!");
    }

    #[test]
    fn mutable_to_immutable_coercion() {
        let mut mybox = MyBox::new( String::from("Hello") );

        let mut_ref = &mut mybox;
        mut_ref.push('!'); // Implicit coercion of `&mut Self` from `&mut MyBox` to `&mut String`.

        // Mutable reference to immutable cast:
        let immut_ref = mut_ref as &String;

        assert_eq!(immut_ref, "Hello!");
    }

}

