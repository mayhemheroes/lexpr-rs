# lexpr: S-expressions for Rust [![Latest Version]][crates.io] [![Rustc Version 1.32+]][rustc]

[Latest Version]: https://img.shields.io/crates/v/lexpr.svg
[crates.io]: https://crates.io/crates/lexpr
[Rustc Version 1.32+]: https://img.shields.io/badge/rustc-1.32+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html

```toml
[dependencies]
lexpr = "0.2.3"
```

You may be looking for:

- [API Documentation](https://docs.rs/crate/lexpr/)
- [API Documentation for master branch](https://rotty.github.io/lexpr-rs/master/lexpr/)
- [Serde support for S-expressions](https://github.com/rotty/lexpr-rs/serde-lexpr)
- [TODO](./TODO.md)
- [Goals and a survey of other S-expression crates](./docs/why.md)
- [Release Notes](./NEWS.md)

[S-expressions](https://en.wikipedia.org/wiki/S-expression) are the
human-readable, textual representation of code and data in the Lisp
family of languages. `lexpr` aims to provide the tools to:

- Embed S-expression data into Rust programs using the `sexp` macro:

  ```rust
  use lexpr::sexp;

  let address = sexp!(((name . "Jane Doe") (street . "4026 Poe Lane")));
  ```

- Construct and destructure S-expression data using a full-featured
  API:

  ```rust
  use lexpr::Value;

  let names = Value::list(vec!["Alice", "Bob", "Mallory"]);
  println!("The bad guy is {}", names[2].as_str().unwrap());
  ```

- Parse and serialize S-expression data from and to its textual
  representation.

To get a better idea of the direction `lexpr` is headed, you may want
to take at the [TODO](./TODO.md) or the ["why"](./docs/why.md)
document.

## Supported Lisp dialects

Currently, `lexpr` focuses on Scheme, mostly based on R6RS and R7RS
syntax, with some extensions, and Emacs Lisp. The following features,
common across dialects, are not yet implemented:

- Syntactic shorthands for `quote`, `quasiquote`, `unquote` and
  `unquote-splicing`. Again, these are not usually important when
  using S-expressions as a data exchange format.
- Support for number syntax is currently quite limited. Integers and
  floating point values written in decimal notation should work
  though.

Further dialect-specific omissions, both ones that are planned to be
fixed in the future, and deliberate ones, are listed below. If you are
missing a feature that is not yet listed here, please [file an
issue]!.

### Scheme

- For strings, continuation line syntax (using a trailing slash) is
  not yet implemented.
- Block comments.
- Directives, such as `#!fold-case` and `#!no-fold-case` are not
  implemented. It's not clear if these will be implemented at all.

### Emacs Lisp

The main area where `lexpr` only supports a subset of Emacs Lisp are
strings and characters. You can have a look at [the gory
details](./docs/elisp-strings.md).

## Licensing

The code and documentation in the `lexpr` crate is [free
software](https://www.gnu.org/philosophy/free-sw.html), dual-licensed
under the [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE)
license, at your choosing.

The `lexpr` repository contains code and documentation adapted from
the following projects:

- [`serde_json`](https://github.com/serde-rs/json), also dual-licensed
  under MIT/Apache-2.0 licenses.
- [`sexpr`](https://github.com/zv/sexpr), Copyright 2017 Zephyr
  Pellerin, dual-licensed under the same licenses.