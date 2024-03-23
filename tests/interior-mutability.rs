
/// Tests the Interior Mutability pattern by means of the `RefCell` smart pointer.
/// (Rust Book section 15.5)
///
#[cfg(test)]
mod test_interior_mutability {

    /// Inteface for an object able to transmit messages somewhere.
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    /// Tracks usage for some kind of resource, sending warning
    /// messages through a `Messenger` when some limits are reached.
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T, // The `Messenger` must be alive all the time the `LimitTracker` is.
        value: usize,   // Current usage level
        max:   usize,   // Limit usage level
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        /// Constructor
        pub fn new(messenger: &'a T, max:usize) -> LimitTracker<'a, T> {
            LimitTracker { messenger, value: 0, max }
        }

        /// Sets a usage value for whaterver resource we are tracking.
        /// Sends high usage warnings when relevant.
        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let usage_percent = self.value as f64 / self.max as f64;
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
        // In order to allow modification the `Vec<String>` (message list) from the `send` function,
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
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // Expect an initial empty message list. `borrow()` returns an immutable reference.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 0);

        // Setting the "value" to 80 out of 100 should results in emitting a warning:
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

