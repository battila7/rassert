use crate::core::{Expectation, ExpectationChain};
use std::cell::Cell;
use std::marker::PhantomData;

pub trait CustomFnExpectationsExt<'a, T> {
    fn to<F: 'a + FnOnce(&T) -> bool>(
        self,
        message: &str,
        expectation_fn: F,
    ) -> ExpectationChain<'a, T>;
}

impl<'a, T> CustomFnExpectationsExt<'a, T> for ExpectationChain<'a, T> {
    fn to<F: 'a + FnOnce(&T) -> bool>(
        self,
        message: &str,
        expectation_fn: F,
    ) -> ExpectationChain<'a, T> {
        self.expecting(ExpectCustomFn {
            message: message.to_owned(),
            expectation_fn: Cell::new(Some(expectation_fn)),
            phantom: PhantomData::default(),
        })
    }
}

struct ExpectCustomFn<T, F: FnOnce(&T) -> bool> {
    message: String,
    expectation_fn: Cell<Option<F>>,
    phantom: PhantomData<T>,
}

impl<'a, T, F> Expectation<T> for ExpectCustomFn<T, F>
where
    F: FnOnce(&T) -> bool,
{
    fn test(&self, actual: &T) -> bool {
        (self.expectation_fn.replace(None).unwrap())(actual)
    }

    fn message(&self, expression: &str, _actual: &T) -> String {
        format!("Expected {} to\n  {}", expression, self.message)
    }
}
