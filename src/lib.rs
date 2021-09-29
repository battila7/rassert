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
        let a: Result<u32, u64> = Err(10);

        expect!(&a)
            .to_be_ok_with(&69)
            .conclude_panic();
    }
}
