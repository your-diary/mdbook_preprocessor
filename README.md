# 1. About

An example preprocessor for `mdbook`.

This converts `==[string]==` into `<font color='red'>[string]</font>`, where the color `red` can be customized (see below).

# 2. Configurations

Add the configuration below to `book.toml`.
```toml
[preprocessor.ynn]
command = "<path to executable>"
```

Optionally, you can specify `color` field to change the target color from the default `red` to any color you want.
```toml
[preprocessor.ynn]
command = "<path to executable>"
color = "blue"
```

# 3. Build

```bash
$ cargo build --release
```

# 4. Test

```bash
$ cargo test
```

# 5. References

- [*Preprocessors - mdBook Documentation*](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html)

- [*Configuring Preprocessors - mdBook Documentation*](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html?highlight=command#provide-your-own-command)

<!-- vim: set spell: -->

