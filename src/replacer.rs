use std::{
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use toml::{Value, value::Table};

struct Pattern<'a> {
    old: &'a str,
    new: &'a str,
}

//replaces Markdown content
//`s`: original Markdown
//`config`: configurations of this preprocessor specified in `book.toml` (`Table` is like `HashMap`)
pub fn replace(filepath: &Option<PathBuf>, s: &str, config: &Table) -> String {
    let mut patterns = vec![];
    match config.get("patterns") {
        Some(Value::Array(l)) => {
            l.iter().for_each(|l| {
                let l = l.as_array().unwrap();
                assert_eq!(2, l.len());
                let pattern = Pattern {
                    old: l[0].as_str().unwrap(),
                    new: l[1].as_str().unwrap(),
                };
                patterns.push(pattern);
            });
        }
        _ => return s.to_owned(),
    };

    let mut s = s.to_owned();
    for pattern in patterns {
        let re = Regex::new(pattern.old).unwrap();
        s = re.replace_all(&s, pattern.new).to_string();
    }

    if let Some(Value::String(format)) = config.get("timestamp")
        && let Some(p) = filepath
    {
        let timestamp = generate_timestamp(p, format);
        s = format!("{}\n{}", timestamp, s);
    }

    s
}

fn generate_timestamp(p: &Path, format: &str) -> String {
    let res = Command::new("git")
        .args([
            "log",
            "-1",
            &format!("--pretty=format:{}", format),
            &format!("src/{}", p.to_str().unwrap()),
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(res.stdout).unwrap();
    let timestamp = stdout.trim().split('\n').next().unwrap();
    format!(
        "<div id='mdbook_preprocessor_last_modified'>{}</div>\n",
        timestamp
    )
}

#[cfg(test)]
mod test {
    use toml::value::Array;

    use super::*;

    #[test]
    fn test01() {
        let config = Table::new();
        assert_eq!(
            ":warning: :check: :unknown: ==abc=def==x==pq==",
            replace(
                &None,
                ":warning: :check: :unknown: ==abc=def==x==pq==",
                &config
            )
        );
    }

    #[test]
    fn test02() {
        let mut config = Table::new();
        let patterns = vec![
            Value::Array(Array::from(vec![
                Value::String(":warning:".to_owned()),
                Value::String("⚠️".to_owned()),
            ])),
            Value::Array(Array::from(vec![
                Value::String(":check:".to_owned()),
                Value::String("✅".to_owned()),
            ])),
            Value::Array(Array::from(vec![
                Value::String("==(.*?)==".to_owned()),
                Value::String("<font color=Red>$1</font>".to_owned()),
            ])),
        ];
        config.insert("patterns".to_owned(), Value::Array(patterns));

        assert_eq!(
            "⚠️ ✅ :unknown: <font color=Red>abc=def</font>x<font color=Red>pq</font>",
            replace(
                &None,
                ":warning: :check: :unknown: ==abc=def==x==pq==",
                &config
            )
        );
    }
}
