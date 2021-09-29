use crate::core::{Expectation, ExpectationChain};
use std::fmt::Debug;

pub trait VectorExpectationsExt<'a, T> {
    fn to_have_length(self, expected: usize) -> ExpectationChain<'a, Vec<T>>;

    fn to_be_empty(self) -> ExpectationChain<'a, Vec<T>>;

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

pub trait VectorItemExpectationsExt<'a, T>
where
    T: Debug + PartialEq,
{
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
