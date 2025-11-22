use official_mdbook_preprocessor::book::Book;
use official_mdbook_preprocessor::book::BookItem;
use official_mdbook_preprocessor::errors::Error;
use official_mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use toml::value::Table;

use crate::replacer;

pub struct MyPreprocessor;

impl MyPreprocessor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Preprocessor for MyPreprocessor {
    fn name(&self) -> &str {
        "ynn"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = match ctx
            .config
            .get::<Table>(&format!("preprocessor.{}", self.name()))?
        {
            Some(t) => t.clone(),
            None => Table::new(),
        };

        book.for_each_mut(|e: &mut BookItem| {
            if let BookItem::Chapter(c) = e {
                c.content = replacer::replace(&c.source_path, &c.content, &config);
            }
        });

        Ok(book)
    }
}
