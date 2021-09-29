use crate::core::{Expectation, ExpectationChain};

pub trait BoolExpectationsExt<'a> {
    fn to_be_true(self) -> ExpectationChain<'a, bool>;

    fn to_be_false(self) -> ExpectationChain<'a, bool>;
}

impl<'a> BoolExpectationsExt<'a> for ExpectationChain<'a, bool> {
    fn to_be_true(self) -> ExpectationChain<'a, bool> {
        self.expecting(ExpectTrue {})
    }

    fn to_be_false(self) -> ExpectationChain<'a, bool> {
        self.expecting(ExpectFalse {})
    }
}

struct ExpectTrue {}

impl Expectation<bool> for ExpectTrue {
    fn test(&self, actual: &bool) -> bool {
        *actual
    }

    fn message(&self, expression: &str, _actual: &bool) -> String {
        format!("Expected {}\n  to be true.", expression)
    }
}

struct ExpectFalse {}

impl Expectation<bool> for ExpectFalse {
    fn test(&self, actual: &bool) -> bool {
        !*actual
    }

    fn message(&self, expression: &str, _actual: &bool) -> String {
        format!("Expected {}\n  to be false.", expression)
    }
}
