# 1. About

An example preprocessor for `mdbook`.

# 2. Configurations

## 2.1 Configurations

Add the configuration below to `book.toml`.
```toml
[preprocessor.ynn]
command = "<path to executable>"
patterns = [
    ["<old>", "<new>"],
    ...
]
timestamp = "<format>"
```

- `patterns` (optional)

    `patterns` provides a simple replace functionality with user-defined regular expressions.

    - `"<old>"` is a regular expression.

    - `"<new>"` is generally a string literal, but `$0`, `$1`, ... can be used to represent capture groups.

- `timestamp` (optional)

    When `timestamp` is specified, the last commit date of the source file is prepended to each page.

    The prepended timestamp is enclosed in `<div id='mdbook_preprocessor_last_modified'></div>`, making it customizable via CSS.

    Commit dates are retrieved this command:
    ```bash
    git log -1 --pretty='format:<format>' <file>
    ```

## 2.2 Example

```toml
[preprocessor.ynn]
command = "/Users/user/.cargo/bin/mdbook_preprocessor"
patterns = [
    [":warning:", "⚠️"],
    [":check:", "✅"],
    ["==(.*?)==", "<font color=Red>$1</font>"],
]
timestamp = "🕒 last modified: %cs"
```

The last pattern converts `==[string]==` into `<font color=Red>[string]</font>`.

# 3. Installation

## 3.1 From Local

```bash
$ cargo install --locked --path .
```

## 3.2 From GitHub

```bash
$ cargo install --locked --git 'https://github.com/your-diary/mdbook_preprocessor'
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
pub fn replace(filepath: &Option<PathBuf>, s: &str, config: &Table) -> String {
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

