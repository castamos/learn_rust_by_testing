
#[cfg(test)]
mod test_refcell {

    use std::cell::RefCell;

    /// Simple checks for borrowing through a `RefCell`
    #[test]
    fn it_allows_modification_of_immutable_ref() {

        let message_cell = RefCell::new( String::from("Hello") );
        assert_eq!(*message_cell.borrow(), "Hello");

        {
            let mut inner_string = message_cell.borrow_mut();
            (*inner_string).push_str(" world!");
        }

        assert_eq!(*message_cell.borrow(), "Hello world!");
    }

    /// Negative test to panic when attempting to borrow a data from a `RefCell`
    /// when it was already borrowed mutably.
    #[test]
    #[should_panic]
    fn it_panics_on_borrowing_a_mutable_borrow() {
        let message_cell = RefCell::new( String::from("Hello") );
        assert_eq!(*message_cell.borrow(), "Hello");
        
        let mut inner_string_mut = message_cell.borrow_mut();
        (*inner_string_mut).push_str(" world!");
        
        let inner_string = message_cell.borrow();
        assert_eq!(*inner_string, "Hello world!");
    }
}


/// Tests the Interior Mutability pattern by means of the `RefCell` smart pointer.
/// [Rust Book section 15.5]
///
/// In this example we pass an immutable reference to the constructor of another object.
/// Then, that object calls a method of the immutable reference, but that method actually
/// changes the internal state of the object. This is accomplished by wrapping the data
/// to change in a `RefCell` smart pointer, which allows getting mutable references to the
/// underlying data.
///
/// Borrow checking is still performed when using `RefCell`. However, this checking is done
/// at run time instead of at compile time; therefore, there is some performance penalty
/// we have to pay in order to get more flexibility.
///
#[cfg(test)]
mod test_interior_mutability {

    /// Inteface for an object able to `send` messages "somewhere".
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    /// Tracks usage for some kind of resource, sending warning
    /// messages through a `Messenger` when specific limits are reached.
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T, // The `Messenger` must be alive all the time the `LimitTracker` is.
        usage:     usize, // Current usage level
        max_usage: usize, // Limit usage level; will start sending warnings when reached.
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        /// Constructor
        pub fn new(messenger: &'a T, max_usage: usize) -> LimitTracker<'a, T> {
            LimitTracker { messenger, usage: 0, max_usage }
        }

        /// Sets a usage value for whaterver resource we are tracking.
        /// Sends high usage warnings when reaching `max_usage`.
        pub fn set_value(&mut self, usage: usize) {
            self.usage = usage;

            let usage_percent = self.usage as f64 / self.max_usage as f64;
            if usage_percent >= 1.0 {
                self.messenger.send("ERROR: Quota exceeded.");
            }
            else if usage_percent >= 0.9 {
                self.messenger.send("WARNING: Reached 90% of quota.");
            }
            else if usage_percent >= 0.75 {
                self.messenger.send("INFO: Reached 75% of quota.");
            }
        }
    }

    use std::cell::RefCell;

    /// An implementation for a `Messenger` that just saves the messages internally
    /// (does not send them anywhare).
    struct MockMessenger {
        // In order to allow modification the of the `Vec<String>` (message list) from the `send` function,
        // which, according to the implemented `Messenger` trait, recieves an immutable reference to
        // `self`, we wrap the `Vec<String>` in a `RefCell`:
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        /// Constructor
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // `borrow_mut()` returns a mutable reference even though `self` is immutable.
            // In this scenario, borrow checking is performed at run time.
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_mutates_internal_field_through_immutable_ref() {
        let mock_messenger = MockMessenger::new(); // Note this variable is not mutable.
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // Expect an initial empty message list. `borrow()` returns an immutable reference.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 0);

        // Setting the "value" to 80 out of 100 should result in emitting a warning:
        limit_tracker.set_value(80);
        // Note some inner state of `mock_messenger` is changed even though it's not mutable:
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


/// Tests the Multiple Interior Mutability pattern.
/// [Rust Book section 15.5]
///
/// Here we have a datastructure able to share mutable elements with other instances of the
/// datastructure. Reference counting (`Rc`) ensures shared elements are freed when no longer
/// referenced; using `RefCell` we gain the ability to mutate the shared elements.
///
#[cfg(test)]
mod test_multiple_interior_mutability {

    /// A `Cons` list (recursive list implementation).
    /// Values are wrapped by `RefCell`s so that we can get mutable reference to them.
    ///
    /// Also both list components ("head" and "tail") are wrapped in `Rc`s, so that
    /// they can be owned by multiple lists, i.e., so that multiple lists can share
    /// the same elements by reference.
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_makes_multiple_refs() {

        // `RefCell` is only for mutability
        let start_value = Rc::new(RefCell::new(5));

        // A list (with only one element) that will be the common tail
        // of two different lists:
        let common_list = Rc::new(
            Cons(
                Rc::clone(&start_value),
                Rc::new(Nil),
            )
        );

        // A list that "contains" `common_list` by reference (as its "tail") and also owns
        // a new element ("head").
        let branch_1 = Rc::new(
            Cons(
                Rc::new(RefCell::new(3)),
                Rc::clone(&common_list),
            )
        );

        // Another list that "contains" `common_list` by reference (as its "tail") and also owns
        // a new element ("head").
        let branch_2 = Rc::new(
            Cons(
                Rc::new(RefCell::new(4)),
                Rc::clone(&common_list),
            )
        );
            
        // Check the lists are `[3, 5]` and `[4, 5]`; where the element `5` comes from the common list:
        assert_eq!(format!("{:?}", branch_1), "Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Nil))");
        assert_eq!(format!("{:?}", branch_2), "Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Nil))");

        // Change the common list suffix by means of a mutable borrow:
        *(start_value.borrow_mut()) = 10;

        // Ensure the "derived" lists reflect the change in their common element: `[3, 10]` and `[4, 10]`:
        assert_eq!(format!("{:?}", branch_1), "Cons(RefCell { value: 3 }, Cons(RefCell { value: 10 }, Nil))");
        assert_eq!(format!("{:?}", branch_2), "Cons(RefCell { value: 4 }, Cons(RefCell { value: 10 }, Nil))");
    }
}
