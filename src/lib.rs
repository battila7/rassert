mod core;
mod expectations;

pub use crate::core::{Expectation, ExpectationChain, ExpressionUnderTest, SourceLocation};

pub mod prelude {
    pub use super::expect;
    pub use super::expect_matches;

    pub use super::expectations::*;
}
