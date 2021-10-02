use crate::core::{Expectation, ExpectationChain};
use std::fmt::Debug;

/// Expectation extension for working with [Result]s.
pub trait ResultExpectationsExt<'a, T, E> {
    /// Asserts that the tested [Result] is `Ok(..)`.
    fn to_be_ok(self) -> ExpectationChain<'a, Result<T, E>>;

    /// Asserts that the tested [Result] is `Err(..)`.
    fn to_be_err(self) -> ExpectationChain<'a, Result<T, E>>;
}

impl<'a, T, E> ResultExpectationsExt<'a, T, E> for ExpectationChain<'a, Result<T, E>> {
    fn to_be_ok(self) -> ExpectationChain<'a, Result<T, E>> {
        self.expecting(ExpectResultOk {})
    }

    fn to_be_err(self) -> ExpectationChain<'a, Result<T, E>> {
        self.expecting(ExpectResultErr {})
    }
}

struct ExpectResultOk {}

impl<T, E> Expectation<Result<T, E>> for ExpectResultOk {
    fn test(&self, actual: &Result<T, E>) -> bool {
        actual.is_ok()
    }

    fn message(&self, expression: &str, _actual: &Result<T, E>) -> String {
        format!("Expected Result {}\n  to be Ok(..).", expression)
    }
}

struct ExpectResultErr {}

impl<T, E> Expectation<Result<T, E>> for ExpectResultErr {
    fn test(&self, actual: &Result<T, E>) -> bool {
        actual.is_err()
    }

    fn message(&self, expression: &str, _actual: &Result<T, E>) -> String {
        format!("Expected Result {}\n  to be Err(..).", expression)
    }
}

/// Expectation extension to assert the contents of an [Result] if the
/// contained `Ok` type is [Debug] and [PartialEq].
pub trait ResultOkExpectationsExt<'a, T, E>
where
    T: Debug + PartialEq,
{
    fn to_be_ok_with(self, expected: &'a T) -> ExpectationChain<'a, Result<T, E>>;
}

impl<'a, T, E> ResultOkExpectationsExt<'a, T, E> for ExpectationChain<'a, Result<T, E>>
where
    T: Debug + PartialEq,
{
    fn to_be_ok_with(self, expected: &'a T) -> ExpectationChain<'a, Result<T, E>> {
        self.expecting(ExpectResultOkWith { item: expected })
    }
}

struct ExpectResultOkWith<'a, T>
where
    T: Debug + PartialEq,
{
    item: &'a T,
}

impl<'a, T, E> Expectation<Result<T, E>> for ExpectResultOkWith<'a, T>
where
    T: Debug + PartialEq,
{
    fn test(&self, actual: &Result<T, E>) -> bool {
        actual
            .as_ref()
            .map_or(false, |contained| contained == self.item)
    }

    fn message(&self, expression: &str, actual: &Result<T, E>) -> String {
        actual.as_ref().map_or(
            format!(
                "Expected Result {}\n  to be Ok({:?})\n  but was Err",
                expression, self.item
            ),
            |contained| {
                format!(
                    "Expected Result {}\n  to be Ok({:?})\n  but was Ok({:?})",
                    expression, self.item, contained
                )
            },
        )
    }
}
