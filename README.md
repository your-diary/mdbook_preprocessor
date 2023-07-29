# 1. About

An example preprocessor for `mdbook`.

This provides a simple replace functionality with user-defined regular expressions.

# 2. Configurations

## 2.1 Configurations

Add the configuration below to `book.toml`.
```toml
[preprocessor.ynn]
command = "<path to executable>"
patterns = [
    ["<old>", "<new>"]
]
```

- Multiple `["<old>", "<new>"]` can be specified.

- `"<old>"` is a regular expression.

- `"<new>"` is generally a string literal, but `$0`, `$1`, ... can be used to represent capture groups.

- `patterns` itself can be omitted, in which case this preprocessor does nothing.

## 2.2 Example

```toml
[preprocessor.ynn]
command = "/Users/user/.cargo/bin/mdbook_preprocessor"
patterns = [
    [":warning:", "⚠️"],
    [":check:", "✅"],
    ["==(.*?)==", "<font color=Red>$1</font>"],
]
```

The last pattern converts `==[string]==` into `<font color=Red>[string]</font>`.

# 3. Installation

## 3.1 From Local

```bash
$ cargo install --path .
```

## 3.2 From GitHub

```bash
$ cargo install --git 'https://github.com/your-diary/mdbook_preprocessor'
```

# 4. Build

```bash
$ cargo build --release
```

# 5. Test

```bash
$ cargo test
```

# 6. How To Customize This Project

Basically, the only part you may want to edit is the body of the `replace()` function in [`src/replacer.rs`](./src/replacer.rs).

```rust
fn replace(s: &str, config: &Table) -> String {
    /* ... */
}
```

Optionally, you may want to fix or add its tests.

For a more advanced but yet simple example, [the official example preprocessor](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html#hooking-into-mdbook) would help.

# 7. References

- [*Preprocessors - mdBook Documentation*](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html)

- [*Configuring Preprocessors - mdBook Documentation*](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html?highlight=command#provide-your-own-command)

- [*TOML: Tom's Obvious Minimal Language*](https://toml.io/en/)

<!-- vim: set spell: -->

