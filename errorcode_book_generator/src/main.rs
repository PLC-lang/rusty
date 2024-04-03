use clap::{Arg, Command};
use mdbook::preprocess::Preprocessor;

///! A mdbook preprocessor to generate error code pages

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

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else {

    }
}

struct ErrorCodeGenerator {}

impl Preprocessor for ErrorCodeGenerator {
    fn name(&self) -> &str {
        "Error codes generator"
    }

    fn run(&self, ctx: &mdbook::preprocess::PreprocessorContext, book: mdbook::book::Book) -> mdbook::errors::Result<mdbook::book::Book> {
        //Get a config location on where to read the errors from
        //Get a config location on where to place the error chapters in the book
    }
}

#[cfg(test)]
mod tests {

}
