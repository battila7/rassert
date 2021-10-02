use crate::core::{Expectation, ExpectationChain};
use std::fmt::Debug;

/// Expectation extension for working with [Vec]s.
pub trait VectorExpectationsExt<'a, T> {
    /// Asserts that the length of the tested vector is equal to [expected].
    fn to_have_length(self, expected: usize) -> ExpectationChain<'a, Vec<T>>;

    /// Asserts that the tested vector is empty.
    fn to_be_empty(self) -> ExpectationChain<'a, Vec<T>>;

    /// Asserts that the tested vector is not empty (it contains at least one item).
    fn to_be_non_empty(self) -> ExpectationChain<'a, Vec<T>>;
}

impl<'a, T> VectorExpectationsExt<'a, T> for ExpectationChain<'a, Vec<T>> {
    fn to_have_length(self, expected: usize) -> ExpectationChain<'a, Vec<T>> {
        self.expecting(ExpectVectorLength {
            expected_length: expected,
        })
    }

    fn to_be_empty(self) -> ExpectationChain<'a, Vec<T>> {
        self.expecting(ExpectVectorEmpty {})
    }

    fn to_be_non_empty(self) -> ExpectationChain<'a, Vec<T>> {
        self.expecting(ExpectVectorNonEmpty {})
    }
}

struct ExpectVectorLength {
    expected_length: usize,
}

impl<T> Expectation<Vec<T>> for ExpectVectorLength {
    fn test(&self, actual: &Vec<T>) -> bool {
        actual.len() == self.expected_length
    }

    fn message(&self, expression: &str, actual: &Vec<T>) -> String {
        format!(
            "Expected vec {}\n  to have length {}\n  but was of length {}",
            expression,
            self.expected_length,
            actual.len()
        )
    }
}

struct ExpectVectorEmpty {}

impl<T> Expectation<Vec<T>> for ExpectVectorEmpty {
    fn test(&self, actual: &Vec<T>) -> bool {
        actual.is_empty()
    }

    fn message(&self, expression: &str, _actual: &Vec<T>) -> String {
        format!("Expected vec {}\n  to be empty.", expression)
    }
}

struct ExpectVectorNonEmpty {}

impl<T> Expectation<Vec<T>> for ExpectVectorNonEmpty {
    fn test(&self, actual: &Vec<T>) -> bool {
        !actual.is_empty()
    }

    fn message(&self, expression: &str, _actual: &Vec<T>) -> String {
        format!("Expected vec {}\n  to be non-empty.", expression)
    }
}

/// Expectation extension for working with the items of a [Vec] if the
/// contained type is [Debug] and [PartialEq].
pub trait VectorItemExpectationsExt<'a, T>
where
    T: Debug + PartialEq,
{
    /// Asserts that the tested vec contains [expected].
    fn to_contain(self, expected: &'a T) -> ExpectationChain<'a, Vec<T>>;
}

impl<'a, T> VectorItemExpectationsExt<'a, T> for ExpectationChain<'a, Vec<T>>
where
    T: Debug + PartialEq,
{
    fn to_contain(self, expected: &'a T) -> ExpectationChain<'a, Vec<T>> {
        self.expecting(ExpectVectorContain { item: expected })
    }
}

struct ExpectVectorContain<'a, T>
where
    T: Debug + PartialEq,
{
    item: &'a T,
}

impl<'a, T> Expectation<Vec<T>> for ExpectVectorContain<'a, T>
where
    T: Debug + PartialEq,
{
    fn test(&self, actual: &Vec<T>) -> bool {
        actual.contains(self.item)
    }

    fn message(&self, expression: &str, _actual: &Vec<T>) -> String {
        format!("Expected vec {}\n  to contain {:?}", expression, self.item)
    }
}
