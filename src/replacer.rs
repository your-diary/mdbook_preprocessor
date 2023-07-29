use regex::Regex;
use toml::{Table, Value};

struct Pattern<'a> {
    old: &'a str,
    new: &'a str,
}

//replaces Markdown content
//`s`: original Markdown
//`config`: configurations of this preprocessor specified in `book.toml` (`Table` is like `HashMap`)
pub fn replace(s: &str, config: &Table) -> String {
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

    s
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
            replace(":warning: :check: :unknown: ==abc=def==x==pq==", &config)
        );
    }

    #[test]
    fn test02() {
        let mut config = Table::new();
        let mut patterns = Array::new();
        patterns.push(Value::Array(Array::from(vec![
            Value::String(":warning:".to_owned()),
            Value::String("⚠️".to_owned()),
        ])));
        patterns.push(Value::Array(Array::from(vec![
            Value::String(":check:".to_owned()),
            Value::String("✅".to_owned()),
        ])));
        patterns.push(Value::Array(Array::from(vec![
            Value::String("==(.*?)==".to_owned()),
            Value::String("<font color=Red>$1</font>".to_owned()),
        ])));
        config.insert("patterns".to_owned(), Value::Array(patterns));

        assert_eq!(
            "⚠️ ✅ :unknown: <font color=Red>abc=def</font>x<font color=Red>pq</font>",
            replace(":warning: :check: :unknown: ==abc=def==x==pq==", &config)
        );
    }
}
