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
        let expected = 20;

        let _ = expect!(&12).to_equal(&expected).conclude_panic();
    }
}
