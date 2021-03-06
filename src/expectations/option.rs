use crate::core::{Expectation, ExpectationChain};
use std::fmt::Debug;

/// Expectation extension for working with [Option]s.
pub trait OptionExpectationsExt<'a, T> {
    /// Asserts that the tested [Option] holds `Some` value.
    fn to_be_some(self) -> ExpectationChain<'a, Option<T>>;

    /// Asserts that the tested [Option] is `None`.
    fn to_be_none(self) -> ExpectationChain<'a, Option<T>>;
}

impl<'a, T> OptionExpectationsExt<'a, T> for ExpectationChain<'a, Option<T>> {
    fn to_be_some(self) -> ExpectationChain<'a, Option<T>> {
        self.expecting(ExpectSome {})
    }

    fn to_be_none(self) -> ExpectationChain<'a, Option<T>> {
        self.expecting(ExpectNone {})
    }
}

struct ExpectSome {}

impl<T> Expectation<Option<T>> for ExpectSome {
    fn test(&self, actual: &Option<T>) -> bool {
        actual.is_some()
    }

    fn message(&self, expression: &str, _actual: &Option<T>) -> String {
        format!("Expected Option {}\n  to be Some(..).", expression)
    }
}

struct ExpectNone {}

impl<T> Expectation<Option<T>> for ExpectNone {
    fn test(&self, actual: &Option<T>) -> bool {
        actual.is_none()
    }

    fn message(&self, expression: &str, _actual: &Option<T>) -> String {
        format!("Expected Option {}\n  to be None.", expression)
    }
}

/// Expectation extension to assert the contents of an [Option] if the
/// contained type is [Debug] and [PartialEq].
pub trait OptionItemExpectationsExt<'a, T>
where
    T: Debug + PartialEq,
{
    /// Asserts that the actual [Option] holds `Some(expected)`.
    fn to_contain(self, expected: &'a T) -> ExpectationChain<'a, Option<T>>;
}

impl<'a, T> OptionItemExpectationsExt<'a, T> for ExpectationChain<'a, Option<T>>
where
    T: Debug + PartialEq,
{
    fn to_contain(self, expected: &'a T) -> ExpectationChain<'a, Option<T>> {
        self.expecting(ExpectOptionContain { item: expected })
    }
}

struct ExpectOptionContain<'a, T>
where
    T: Debug + PartialEq,
{
    item: &'a T,
}

impl<'a, T> Expectation<Option<T>> for ExpectOptionContain<'a, T>
where
    T: Debug + PartialEq,
{
    fn test(&self, actual: &Option<T>) -> bool {
        actual
            .as_ref()
            .map(|contained| contained == self.item)
            .unwrap_or(false)
    }

    fn message(&self, expression: &str, actual: &Option<T>) -> String {
        format!(
            "Expected Option {}\n  to contain {:?}\n  but held {:?}",
            expression, self.item, actual
        )
    }
}
