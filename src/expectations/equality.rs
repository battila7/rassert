use crate::core::{Expectation, ExpectationChain};
use std::fmt::Debug;

/// Expectation extension for values that are [PartialEq] and [Debug].
pub trait EqualityExpectationsExt<'a, T> {
    /// Asserts that the actual value is equal to the [expected] value. The same
    /// as the `to_be` function.
    fn to_equal(self, expected: &'a T) -> ExpectationChain<'a, T>;

    /// Asserts that the actual value is equal to the [expected] value. The same
    /// as the `to_equal` function.
    fn to_be(self, expected: &'a T) -> ExpectationChain<'a, T>;

    /// Asserts that the actual value is not equal to the [expected] value. The same
    /// as the `to_not_be` function.
    fn to_not_equal(self, expected: &'a T) -> ExpectationChain<'a, T>;

    /// Asserts that the actual value is not equal to the [expected] value. The same
    /// as the `to_not_equal` function.
    fn to_not_be(self, expected: &'a T) -> ExpectationChain<'a, T>;
}

impl<'a, T> EqualityExpectationsExt<'a, T> for ExpectationChain<'a, T>
where
    T: PartialEq + Debug,
{
    fn to_equal(self, expected: &'a T) -> ExpectationChain<'a, T> {
        self.expecting(ExpectEquals { expected })
    }

    fn to_be(self, expected: &'a T) -> ExpectationChain<'a, T> {
        self.to_equal(expected)
    }

    fn to_not_equal(self, expected: &'a T) -> ExpectationChain<'a, T> {
        self.expecting(ExpectNotEquals { expected })
    }

    fn to_not_be(self, expected: &'a T) -> ExpectationChain<'a, T> {
        self.to_not_equal(expected)
    }
}

struct ExpectEquals<'a, T: PartialEq + Debug> {
    expected: &'a T,
}

impl<'a, T> Expectation<T> for ExpectEquals<'a, T>
where
    T: PartialEq + Debug,
{
    fn test(&self, actual: &T) -> bool {
        actual == self.expected
    }

    fn message(&self, expression: &str, actual: &T) -> String {
        format!(
            "Expected {}\n  to be {:?}\n  but was {:?}",
            expression, self.expected, actual
        )
    }
}

struct ExpectNotEquals<'a, T: PartialEq + Debug> {
    expected: &'a T,
}

impl<'a, T> Expectation<T> for ExpectNotEquals<'a, T>
where
    T: PartialEq + Debug,
{
    fn test(&self, actual: &T) -> bool {
        actual != self.expected
    }

    fn message(&self, expression: &str, actual: &T) -> String {
        format!(
            "Expected {}\n  to NOT be {:?}\n  but was {:?}",
            expression, self.expected, actual
        )
    }
}
