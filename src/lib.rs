use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::Regex;
use toml::{Table, Value};

pub struct MyPreprocessor;

impl MyPreprocessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Preprocessor for MyPreprocessor {
    fn name(&self) -> &str {
        "my-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config: Table = match ctx.config.get_preprocessor(self.name()) {
            Some(t) => t.clone(),
            None => Table::new(),
        };

        book.for_each_mut(|e: &mut BookItem| {
            if let BookItem::Chapter(c) = e {
                c.content = replace(&c.content, &config);
            }
        });

        Ok(book)
    }
}

fn replace(s: &str, config: &Table) -> String {
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
    use mdbook::preprocess::CmdPreprocessor;

    use super::*;

    #[test]
    fn test01() {
        let input = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "my-preprocessor": {
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "==abc=def==x==pq==",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;

        let expected = {
            let re = Regex::new(r#""content": "[^"]+""#).unwrap();
            let s = re
                .replace_all(
                    input,
                    r#""content": "<font color='Red'>abc=def</font>x<font color='Red'>pq</font>""#,
                )
                .to_string();
            let (_ctx, book) = CmdPreprocessor::parse_input(s.as_bytes()).unwrap();
            book
        };

        let actual = {
            let (ctx, book) = CmdPreprocessor::parse_input(input.as_bytes()).unwrap();
            let result = MyPreprocessor::new().run(&ctx, book);
            assert!(result.is_ok());
            result.unwrap()
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test02() {
        let input = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "my-preprocessor": {
                                "color": "Blue"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "==abc=def==x==pq==",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;

        let expected = {
            let re = Regex::new(r#""content": "[^"]+""#).unwrap();
            let s = re
                .replace_all(
                    input,
                    r#""content": "<font color='Blue'>abc=def</font>x<font color='Blue'>pq</font>""#,
                )
                .to_string();
            let (_ctx, book) = CmdPreprocessor::parse_input(s.as_bytes()).unwrap();
            book
        };

        let actual = {
            let (ctx, book) = CmdPreprocessor::parse_input(input.as_bytes()).unwrap();
            let result = MyPreprocessor::new().run(&ctx, book);
            assert!(result.is_ok());
            result.unwrap()
        };

        assert_eq!(expected, actual);
    }
}
