//! Adapter driving the `plc format` subcommand: file-glob expansion, config discovery, and the
//! `--check`/`--stdout`/in-place write modes. Takes plain arguments rather than `plc_driver`'s
//! `CompileParameters` directly, since `plc_driver` depends on this crate - a dependency the
//! other way round would be circular.

use std::path::{Path, PathBuf};

use plc_diagnostics::diagnostics::Diagnostic;

use crate::config::{self, FormatConfig};
use crate::format_file;

pub struct FormatArgs<'a> {
    pub files: &'a [String],
    pub check: bool,
    pub config_path: Option<&'a str>,
    pub dump_config: bool,
    pub stdout: bool,
}

pub fn run(args: FormatArgs) -> Result<(), Diagnostic> {
    if args.dump_config {
        let config = match args.config_path {
            Some(path) => FormatConfig::from_file(Path::new(path))?,
            None => FormatConfig::default(),
        };
        println!("{}", config.to_yaml_string());
        return Ok(());
    }

    if args.check && args.stdout {
        return Err(Diagnostic::new("--check and --stdout cannot be used together".to_string())
            .with_error_code("E001"));
    }

    let paths = expand_files(args.files)?;
    let explicit_config = args.config_path.map(Path::new);

    let mut any_would_change = false;
    for path in &paths {
        let cfg = config::resolve(Some(path), explicit_config)?;
        let outcome = format_file(path, &cfg)?;

        if args.stdout {
            print!("{}", outcome.formatted);
        } else if args.check {
            if outcome.changed {
                any_would_change = true;
                eprintln!("would reformat {}", path.display());
            }
        } else if outcome.changed {
            std::fs::write(path, &outcome.formatted).map_err(|err| {
                Diagnostic::new(format!("could not write {}: {err}", path.display())).with_error_code("E002")
            })?;
        }
    }

    if args.check && any_would_change {
        return Err(
            Diagnostic::new("one or more files would be reformatted".to_string()).with_error_code("E001")
        );
    }
    Ok(())
}

/// Expands glob patterns in `patterns`. A pattern that is not a glob (or one that matched
/// nothing) is kept as a literal path, so a plain filename that does not exist still produces a
/// clear "could not read" error from `format_file` rather than silently formatting nothing.
fn expand_files(patterns: &[String]) -> Result<Vec<PathBuf>, Diagnostic> {
    let mut paths = Vec::new();
    for pattern in patterns {
        let matches: Vec<PathBuf> = glob::glob(pattern)
            .map_err(|err| {
                Diagnostic::new(format!("invalid file pattern {pattern}: {err}")).with_error_code("E001")
            })?
            .filter_map(|entry| entry.ok())
            .collect();
        if matches.is_empty() {
            paths.push(PathBuf::from(pattern));
        } else {
            paths.extend(matches);
        }
    }
    Ok(paths)
}
