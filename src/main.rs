use std::io;
use std::process;

use itertools::Itertools;
use official_mdbook_preprocessor::Preprocessor;
use official_mdbook_preprocessor::book::Book;
use official_mdbook_preprocessor::errors::Error;

use mdbook_preprocessor::preprocessor::MyPreprocessor;

fn main() -> Result<(), Error> {
    let argv = std::env::args().skip(1).collect_vec();
    if !argv.is_empty() {
        assert_eq!("supports", &argv[0]);
        assert_eq!(2, argv.len());
        let _renderer = &argv[1];
        process::exit(0);
    }

    let preprocessor = MyPreprocessor::new();
    let (ctx, book) = official_mdbook_preprocessor::parse_input(io::stdin())?;
    let result: Book = preprocessor.run(&ctx, book)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
