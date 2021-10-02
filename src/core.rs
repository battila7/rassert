use std::fmt::Debug;

/// Starts a new expectation chain for the supplied expression.
#[macro_export]
macro_rules! expect {
    ($tested:expr) => {
        $crate::blank_chain!($tested)
    };
}

// Originates from the assert_matches library https://github.com/murarth/assert_matches
// Licensed under the Apache 2.0 Software License.
#[macro_export]
macro_rules! expect_matches {
    ( $e:expr , $($pat:pat)|+ ) => {
        match $e {
            $($pat)|+ => $crate::blank_chain!($e),
            ref e => $crate::failure_chain!($e, format!("Expected {:?} to match {}",
                e, stringify!($($pat)|+)))
        }
    };
    ( $e:expr , $($pat:pat)|+ if $cond:expr ) => {
        match $e {
            $($pat)|+ if $cond => $crate::blank_chain!($e),
            ref e => $crate::failure_chain!($e, format!("Expected {:?} to match {}",
                e, stringify!($($pat)|+ if $cond)))
        }
    };
    ( $e:expr , $($pat:pat)|+ , $($arg:tt)* ) => {
        match $e {
            $($pat)|+ => $crate::blank_chain!($e),
            ref e => $crate::failure_chain!($e, format!("Expected {:?} to match {}: {}",
                e, stringify!($($pat)|+), format_args!($($arg)*)))
        }
    };
    ( $e:expr , $($pat:pat)|+ if $cond:expr , $($arg:tt)* ) => {
        match $e {
            $($pat)|+ if $cond => $crate::blank_chain!($e),
            ref e => $crate::failure_chain!($e, format!("Expected {:?} to match {}: {}",
                e, stringify!($($pat)|+ if $cond), format_args!($($arg)*)))
        }
    };
}

#[macro_export]
macro_rules! blank_chain {
    ($tested:expr) => {
        $crate::core::ExpectationChain::from_expression($crate::core::ExpressionUnderTest {
            actual: $tested,
            tested_expression: std::stringify!($tested),
            location: $crate::core::SourceLocation {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })
    };
}

#[macro_export]
macro_rules! failure_chain {
    ($tested:expr, $message:expr) => {
        $crate::blank_chain!($tested).failure($message)
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SourceLocation {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub struct ExpressionUnderTest<'a, T> {
    pub actual: &'a T,
    pub tested_expression: &'static str,
    pub location: SourceLocation,
}

pub trait Expectation<T> {
    fn test(&self, actual: &T) -> bool;

    fn message(&self, expression: &str, actual: &T) -> String;
}

pub struct ExpectationChain<'a, T> {
    expression: ExpressionUnderTest<'a, T>,

    in_negated_mode: bool,

    negations: Vec<bool>,

    soft_mode: bool,

    expectations: Vec<Box<dyn Expectation<T> + 'a>>,
}

impl<'a, T> ExpectationChain<'a, T> {
    pub fn from_expression(expression: ExpressionUnderTest<'a, T>) -> Self {
        Self {
            expression,
            in_negated_mode: false,
            negations: vec![],
            soft_mode: false,
            expectations: vec![],
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        self.in_negated_mode = !self.in_negated_mode;

        self
    }

    pub fn and(self) -> Self {
        self
    }

    pub fn expecting(mut self, expectation: impl Expectation<T> + 'a) -> Self {
        self.expectations.push(Box::new(expectation));

        self.negations.push(self.in_negated_mode);

        self.in_negated_mode = false;

        self
    }

    pub fn failure(self, message: String) -> Self {
        self.expecting(ExpectMatchFailure {
            preset_message: message,
        })
    }

    pub fn soft(mut self) -> Self {
        self.soft_mode = true;

        self
    }

    pub fn conclude_result(self) -> Result<(), String> {
        let location = self.expression.location;
        let mut message = format!(
            "{}:{}:{}\nwhen testing expression\n\n",
            location.file, location.line, location.column
        );
        message.push_str(&format!("    {}\n\n", self.expression.tested_expression));

        let mut had_failure = false;
        for i in 0..self.expectations.len() {
            let expectation = self.expectations.get(i).unwrap();
            let is_negated = self.negations.get(i).unwrap();

            if !(is_negated ^ expectation.test(self.expression.actual)) {
                had_failure = true;

                let failure_message =
                    expectation.message(self.expression.tested_expression, self.expression.actual);
                let failure_message = if *is_negated {
                    indented("  ", &format!("NOT {}", failure_message))
                } else {
                    indented("  ", &failure_message)
                };

                message.push_str(&failure_message);

                if !self.soft_mode {
                    break;
                }
            }
        }

        if had_failure {
            Err(message)
        } else {
            Ok(())
        }
    }

    pub fn conclude_panic(self) {
        if let Err(message) = self.conclude_result() {
            eprintln!("{}", message);
            panic!()
        }
    }
}

fn indented(indentation: &str, s: &str) -> String {
    let result: Vec<String> = s
        .split('\n')
        .map(|s| format!("{}{}", indentation, s))
        .collect();

    format!("{}\n", result.join("\n"))
}

struct ExpectMatchFailure {
    preset_message: String,
}

impl<T> Expectation<T> for ExpectMatchFailure {
    fn test(&self, _actual: &T) -> bool {
        false
    }

    fn message(&self, _expression: &str, _actual: &T) -> String {
        self.preset_message.clone()
    }
}
