use regex::Regex;
use toml::{Table, Value};

//replaces Markdown content
//`s`: original Markdown
//`config`: configurations of this preprocessor specified in `book.toml` (`Table` is like `HashMap`)
pub fn replace(s: &str, config: &Table) -> String {
    let color = match config.get("color") {
        Some(Value::String(c)) => c,
        _ => "Red",
    };

    let re = Regex::new(r#"==(.*?)=="#).unwrap();
    let s = re
        .replace_all(s, format!(r#"<font color='{}'>$1</font>"#, color))
        .to_string();

    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let config = Table::new();
        assert_eq!(
            "<font color='Red'>abc=def</font>x<font color='Red'>pq</font>",
            replace("==abc=def==x==pq==", &config)
        );
    }

    #[test]
    fn test02() {
        let mut config = Table::new();
        config.insert("color".to_owned(), Value::String("blue".to_owned()));
        assert_eq!(
            "<font color='blue'>abc=def</font>x<font color='blue'>pq</font>",
            replace("==abc=def==x==pq==", &config)
        );
    }
}
