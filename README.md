
# Learning Rust by Testing

The purpose of this project is really for me to learn the Rust programming
language by writting tests for different features of the language itself (and
for some crates).  The project consists of the following:

- Integration tests
- Test utilities needed by the integration tests
- Negative tests for expected compilation failures, provided as doc tests

Perhaps other people may find useful reading and running the tests shared here,
either during their learning process, as a reference for specific details of
Rust, or simply as an example for their own journey applying the "Learn by
Testing" methodology described below.


# Test Organization

All tests are written for the built-in testing framework provided by the
`cargo` utility:

- Integration tests are located/named at: `tests/*.rs`
- Generic test utilities used by integration tests are here: `tests/common/mod.rs`
- Negative compilation tests appear as documentation examples in: `src/lib.rs`

Each integration test file is supposed to contain tests only for a single
feature/topic (of course what a single feature is, is open to interpretation).
Moreover, inside integration test files, closely related tests are grouped into
individual modules; in particular, when several test functions share common
definitions (such as structs, functions, etc), the definitions and test
functions that use them are enclosed in the same module. Note in this latter
scenario that even though cargo sees the tests as "integration", these are
actually unit tests for the common definitions inside the test module.


# How to run the tests and what output to expect

Just run `cargo test`. You should see an output similar to this:

```
   Running unittests src/lib.rs (target/debug/deps/unit-e02d75e0d3443213)
running 0 tests
[...]

	 Running tests/references.rs (target/debug/deps/references-bfc30c13230c4734)
running 13 tests
test test_references::dereference_box ... ok
test test_references::dereference_reference ... ok
[...]
test test_user_defined_deref::mutable_to_immutable_coercion ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Running tests/structs.rs (target/debug/deps/structs-dac00024b28951ae)
running 1 test
test test_structs::tuple_struct_as_type_alias ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[...]

   Doc-tests unit
running 1 test
test src/lib.rs - __doc_test (line 3) - compile fail ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

The output `Running unitttests src/lib.rs` refers to unit tests in
`src/lib.rs`; we don't have any since there is no actual functionality provided
by this crate that we could unit-test. However, `lib.rs` is used for negative
tests for which compilation errors are expected (see below).

Subsequent output blocks of the form `Running tests/*.rs` refer to each of the
actual integration tests that check features of the Rust language (or selected
crates).

Finally, `Doc-tests unit` is the output for documentation tests (living in
`src/lib.rs`), this is example code written inside documentation comments. Since this
test code is inside comments, we use it to safely include negative compilation
tests (i.e. code we expect to fail to compile); for example, to verify that
some code we know violates borrow checking rules actually fails to compile.


# The Learn by Testing Methodology

This learning methodology originally arose from my need to master the Perl
programming language in order to maintain and improve a large CI/CD/QA
codebase.  Perl's design philosophy of _"there's more than one way to do it"_
and _"do what I mean"_ results in a highly expressive language but at the
expense of having lots of edge cases that are sometimes hard to understand
and/or remember.

As we know, an effective way of checking if one is properly understanding any
aspect of a language (or library) is to try it out, which leads to lots of
disposable code snippets.  At some point I found myself writing similar (or
even the same) code snippets for features of the language I did not use often;
therefore, I decided to be more disciplined and started saving and organizing
all those code snippets in a single repository.

I also found that writing the code snippets as unit (or integration) tests was
more effective than just writing a bunch of statements that resulted in
printing "the right answer", mainly because of the following reasons:

1. When we have the need to execute old code snippets it may not be immediately
   obvious why the printed output should be the right answer or if it is really
   the right answer. Tests at least make explicit that in fact that is the
   expected answer.

2. Writing snippets as tests helps develop good testing practices/discipline
   (such as TDD), especially if tests are written for built-in or "de facto"
   test frameworks for the language in question.

Now, this learning approach is really an inverse testing methodology in the
sense that the ultimate source of truth to be considered is the thing under
testing (the compiler/interpreter or library against which we are running the
tests). In other words, these tests are not meant to verify the thing tested
behaves as we think it should; instead they are meant to verify that we, the
learners (assuming a tester role), understand how the thing actually behaves
regardless of if its behavior is "correct" or not.  In short, we take the
saying that _"there are no bugs, just undocumented features"_ to the next
level, because we just want to get thinks done.

