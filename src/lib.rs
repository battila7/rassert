pub mod core;
mod expectations;

pub mod prelude {
    pub use super::expect;

    pub use super::expectations::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works() {
        expect!(&(20 == 20))
            .to_be_false()
            .conclude_panic();
    }
}
