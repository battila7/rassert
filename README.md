<h1 align="center">
  rassert
</h1>

<h3 align="center">
  :crab: :question: :exclamation:
<h3>

<h3 align="center">
  Fluent, easy-to-extend testing assertions for Rust.
</h3>

<p align="center">
  <a href="https://github.com/battila7/rassert/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/battila7/rassert" alt="rassert uses The MIT License.">
  </a>
  <a href="https://crates.io/crates/rassert">
    <img src="https://img.shields.io/crates/v/rassert" alt="Current crates.io version.">
  </a>
  <a href="https://github.com/battila7/rassert/actions/workflows/continuous-integration.yml">
    <img src="https://github.com/battila7/rassert/actions/workflows/continuous-integration.yml/badge.svg" alt="Continuous Integration status.">
  </a>
</p>

## Features

  * **Fluent Expectations.** When writing tests, we strive for clarity and readability. rassert supports this goal by providing a fluent expectation interface allowing one to write assertions as close to natural language as possible. See [Writing Assertions](#writing-assertions).
  * **Human-friendly Failures.** Instead of cryptic failure messages, rassert displays the tested expression, its location, the actual value and the expected value/condition on failure to aid debugging.
  * **Pattern Matching Support.** Use the `expect_matches!` macro to check if an expression matches some pattern, such as `expect_matches!(result, Ok(..))`.
  * **Lazy and Soft Evaluation.**  Assertions are evaluated lazily, only when closing an expectation chain with a `conclude_panic` or `conclude_result` function call. In soft mode (turn on via the `soft()` call), xpectation chain evaluation does not stop on the first failure. See [Writing Assertions](#writing-assertions).
  * **Easy-to-Extend.** Extend rassert with custom expectations easily and succinctly. See [Custom Expectations](#custom-expectations).

## Up and Running

First, add rassert to the `dev-dependencies` section of your `Cargo.toml` file:

~~~~TOML
[dev-dependencies]
rassert = "1"
~~~~

Then simply import the `prelude` module in your tests:

~~~~Rust
#[cfg(test)]
mod tests {
    use rassert::prelude::*;

    #[test]
    fn rassert_works() {
        expect!(true)
            .not()
            .to_be_false()
            .and()
            .to_equal(true)
            .conclude_panic();
    }
}
~~~~

## Writing Assertions

### Expectation Chains

In rassert one can write assertions in the form of expectation chains. Such chains allow for writing multiple expectations against the same expression. There are two so-called entry points with which one can start a chain:

  * `expect!(expression)`
    * Creates a new chain asserting against the provided `expression`.
  * `expect_matches!(expression, pattern)`
    * Creates a new chain from the provided `expression` and automatically adds an expectation to the chain, asserting that `expression` matches `pattern`.

Once a chain is started, one can subsequently call expectations on it, as follows:

~~~~Rust
let v = vec![10];

expect!(v)
    .to_be_non_empty()
    .and()
    .to_contain(10);
~~~~

Note, that the `and()` call is not mandatory, as it only serves readability purposes.

### Concluding Chains

Since rassert evaluates expectations lazily, a chain like the above one will do nothing. A chain will only assert the specified expectations when concluded:

~~~~Rust
// Will panic on a failed expectation.
expect!(true)
    .to_be(false)
    .conclude_panic();

// Will return Result<(), String> containing the error
// message on failure.
let res = expect!(true)
  .to_be(false)
  .conclude_result();
~~~~

### Soft Mode

A chain can be put into soft mode by calling `soft()` prior to concluding the chain:

~~~~Rust
let v = vec![10];

expect!(v)
    .to_contain(15)
    .and()
    .to_contain(20)
    .soft()
    .conclude_panic();
~~~~

Soft chains will not panic/return on the first failure, instead they will run each assertion and present a merged report of every failure that occurred.

### Negating Expectations

One can negate a single subsequent expectation using the `not()` function:

~~~~Rust
expect!(true)
    .not()
    .to_be_false()
    .conclude_panic();
~~~~

If one wishes to negate additionals expectations, then `not()` has to be applied again.

## Available Expectations

  * `T`
    * `to`
  * `T` where `T: Debug + PartialEq`
    * `to_equal`, `to_be`
    * `to_not_equal`, `to_not_be`
  * `boolean`
    * `to_be_true`
    * `to_be_false`
  * `Option<T>`
    * `to_be_some`
    * `to_be_none`
  * `Option<T>` where `T: Debug + PartialEq`
    * `to_contain`
  * `Result<T, E>`
    * `to_be_ok`
    * `to_be_err`˙
  * `Result<T, E>` where `T: Debug + PartialEq`
    * `to_be_ok_with`
  * `Vec<T>`
    * `to_have_length`
    * `to_be_empty`
    * `to_be_non_empty`
  * `Vec<T>` where `T: Debug + PartialEq`
    * `to_contain`

## Custom Expectations

Custom expectations can be written as [extension traits](https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html) on the `ExpectationChain` type, provided by rassert. In what follows, we show how to write custom expectations through an example.

Let's assume, that we want to write an expectation against a custom struct, `Pizza`:

~~~~Rust
#[derive(Debug)]
pub struct Pizza {
    pub flavor: String
}
~~~~

In rassert, expectations are actually structs, following something like a [Command pattern](https://sourcemaking.com/design_patterns/command). Given, we want to check what flavor of Pizza we have, we can create something as follows:

~~~~Rust
use rassert::Expectation;

struct ExpectPizzaFlavor {
    expected_flavor: String
}

impl Expectation<Pizza> for ExpectPizzaFlavor {
    fn test(&self, actual: &Pizza) -> bool {
        actual.flavor.eq(&self.expected_flavor)
    }

    fn message(&self, expression: &str, actual: &Pizza) -> String {
        format!("Expected {:?}\n  to have flavor {}\n  but had {}.", expression, self.expected_flavor, actual.flavor)
    }
}
~~~~

Implementing the `Expectation` trait comes with two functions:

  * `fn(&self, actual) -> bool`
    * The actual assertion which returns true on success and false otherwise. The `actual` parameter corresponds to the value being tested in the chain.
  * `fn(&self, expression, actual)` -> String
    * The message shown if the expectation fails. The `expression` parameter is the stringified expression argument of the `expect!`/`expect_matches!` macro.

Once we got our expectation struct written, we can finally extend the `ExpectationChain` type:

~~~~Rust
use rassert::ExpectationChain;

pub trait PizzaExpectationsExt<'a> {
    fn to_have_flavor(self, expected: &str) -> ExpectationChain<'a, Pizza>;
}

impl<'a> PizzaExpectationsExt for ExpectationChain<'a, Pizza> {
    fn to_have_flavor(self, expected: &str) -> ExpectationChain<'a, Pizza> {
        self.expecting(ExpectPizzaFlavor {
            expected_flavor: expected.to_owned()
        })
    }
}
~~~~

The most important bits of the above snippet are the following:

  * Extension traits must take a generic lifetime parameter and use it in the `ExpectationChain` type returned from expectation functions. This parameter corresponds to the lifetime of the immutable reference held inside the chain. This refernece then refers to the actual tested value.
  * Expectation functions must take `self` since expectation chains are [Consuming builders](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
  * Expectation functions can extend the chain with a new expectation using the `expecting()` function. This function takes an Expectation which will be executed when concluding the chain. The fields of the expectation can be used to parameterize the actual assertion.

The built-in expectations of the [src/expectations](src/expectations) directory also use the above facilities, therefore they serve as a great starting point for writing custom expectations.

## License

Licensed under [The MIT License](LICENSE)
