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
        expect!(&vec!['a'])
            .to_contain(&'b')
            .and()
            .to_contain(&'c')
            .soft()
            .conclude_panic();
    }
}
