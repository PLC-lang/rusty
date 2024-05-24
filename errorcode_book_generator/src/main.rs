use std::{fs, io, path::PathBuf, str::FromStr};

use clap::{Arg, Command};
use mdbook::{
    book::Chapter,
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
    BookItem,
};
use semver::{Version, VersionReq};
use toml::Value;

/// A mdbook preprocessor to generate error code pages

pub fn make_app() -> Command {
    Command::new("error_code_generator")
        .about("A mdbook preprocessor to generate error code pages")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is present by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = ErrorCodeGenerator {};

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn handle_preprocessing(preprocessor: &ErrorCodeGenerator) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            preprocessor.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(preprocessor: &dyn Preprocessor, sub_args: &clap::ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");
    let supported = preprocessor.supports_renderer(renderer);

    if supported {
        std::process::exit(0);
    } else {
        eprintln!("{renderer} is not supported");
        std::process::exit(1);
    }
}

struct ErrorCodeGenerator {}

impl Preprocessor for ErrorCodeGenerator {
    fn name(&self) -> &str {
        "errors"
    }

    fn run(
        &self,
        ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        //Get a config location on where to read the errors from
        //Get a config location on where to place the error chapters in the book
        let preprocessor = ctx.config.get_preprocessor(self.name()).expect("Preprocessor configured");
        let Some((source, target)) = preprocessor
            .get("err_location")
            .and_then(Value::as_str)
            .zip(preprocessor.get("err_target").and_then(Value::as_str))
        else {
            return Err(Error::msg(
                "Invalid configuration, the keys \"err_location\" and \"err_target\" must be set",
            ));
        };

        //Read the config source for all md files
        let source = ctx.root.join(PathBuf::from_str(source)?);
        let target = PathBuf::from_str(target)?;
        if !source.exists() {
            return Err(Error::msg(format!("{}: Invalid Path", source.to_string_lossy())));
        }

        //Find the section in the book where to add the chapters
        let mut files: Vec<_> = fs::read_dir(source)?
            .flatten()
            .map(|it| it.path())
            .filter(|it| it.extension().unwrap_or_default() == "md")
            .collect();
        files.sort();

        // This goes through the entire book to find the correct chapter to add the error codes
        // It then creates sub chapters for each `.md` file it finds
        // The title of the chapter would be the name of the file.
        // The chapters are sorted alphabetically.
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                if chapter.source_path == Some(target.clone()) {
                    let mut parents = chapter.parent_names.clone();
                    parents.push(chapter.name.clone());
                    let mut number = 1;
                    for file in &files {
                        //Add each file to the book
                        let name = file.file_stem().and_then(|it| it.to_str()).unwrap_or_default();
                        let content = std::fs::read_to_string(file).unwrap();
                        let mut error_chapter = Chapter::new(
                            name,
                            content,
                            &target.join(name).with_extension("md"),
                            parents.clone(),
                        );
                        let mut chapter_number = chapter.number.clone().unwrap_or_default();
                        chapter_number.push(number);
                        error_chapter.number = Some(chapter_number);
                        chapter.sub_items.push(error_chapter.into());
                        number += 1;
                    }
                }
            }
        });
        Ok(book)
    }
}
