use std::fmt::Debug;

#[macro_export]
macro_rules! expect {
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
        for expectation in self.expectations {
            if !expectation.test(self.expression.actual) {
                had_failure = true;

                let failure_message =
                    expectation.message(self.expression.tested_expression, self.expression.actual);
                let failure_message = indented("  ", &failure_message);
                message.push_str(&failure_message);

                if self.soft_mode {
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
