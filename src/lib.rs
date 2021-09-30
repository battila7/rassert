mod core;
mod expectations;

pub use crate::core::{Expectation, ExpectationChain, ExpressionUnderTest, SourceLocation};

pub mod prelude {
    pub use super::expect;
    pub use super::expect_matches;

    pub use super::expectations::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works() {
        let a: Result<u32, u64> = Err(10);

        expect_matches!(&a, Ok(c) if c > &10).conclude_panic();
    }
}
