use crate::task::Task;
use anyhow::Result;
use clap::{Parser, Subcommand};
use plc::{ast::SourceRangeFactory, lexer::IdProvider};
use reporter::{BenchmarkReport, ReporterType};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use task::{compile::Compile, run::Run};
use tempfile::{tempdir, TempDir};
use xshell::{cmd, Shell};

mod reporter;
mod task;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Parameters {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    Metrics {
        #[command(subcommand)]
        action: Option<Action>,
        #[arg(value_enum, long, global = true, default_value_t = ReporterType::Sysout)]
        reporter: ReporterType,
    },
}

#[derive(Subcommand, Clone)]
enum Action {
    Run {
        #[arg()]
        directory: String,
    },
    Compile {
        directory: String,
    },
}

fn main() -> anyhow::Result<()> {
    let params = Parameters::parse();
    let Command::Metrics { action, reporter } = params.command;
    run_metrics(action, reporter)?;

    Ok(())
}

fn run_metrics(action: Option<Action>, reporter: ReporterType) -> Result<()> {
    let (work_dir, compiler) = prepare()?;
    //Create tasks
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    match &action {
        Some(Action::Compile { directory }) => {
            for opt in ["none", "less", "default", "aggressive"] {
                let task = Compile {
                    name: directory.clone(),
                    directory: directory.into(),
                    optimization: opt.to_string(),
                    compiler: compiler.clone(),
                };
                tasks.push(Box::new(task));
            }
        }
        Some(Action::Run { directory }) => {
            for opt in ["none", "less", "default", "aggressive"] {
                let task = Run {
                    name: directory.into(),
                    optimization: opt.to_string(),
                    compiler: compiler.clone(),
                    location: directory.into(),
                    parameters: Some("--linker=cc".into()),
                    work_dir: work_dir.path().into(),
                };
                tasks.push(Box::new(task));
            }
        }
        None => {
            let path = work_dir.path();
            tasks.extend(task::get_default_tasks(path, &compiler)?)
        }
    };
    //Run benchmarks
    let mut data = vec![];
    for mut task in tasks {
        println!("Running benchmark for {}", task.get_name());
        let res = task.benchmark(3)?;
        //Report
        data.push((task.get_name(), res));
    }
    data.push(("lexer/lexer.st".into(), Duration::from_micros(benchmark_lexer() as u64)));
    //Reprort data
    let report = BenchmarkReport::new(data)?;
    let reporter = reporter::from_type(reporter);
    reporter.persist(report)?;
    Ok(())
}

fn benchmark_lexer() -> u128 {
    let mut avg = 0;
    let content = std::fs::read_to_string("./xtask/res/lexer.st").unwrap();

    for _ in 0..100 {
        let mut lexer =
            plc::lexer::lex_with_ids(&content, IdProvider::default(), SourceRangeFactory::internal());

        let now = Instant::now();
        while !lexer.is_end_of_stream() {
            lexer.advance();
        }
        avg += now.elapsed().as_micros();

        assert_eq!(lexer.token, plc::lexer::Token::End);
        assert_eq!(lexer.last_range, 139145..139156);
    }

    avg = avg / 100;
    println!("{avg}");
    avg
}

fn prepare() -> Result<(TempDir, PathBuf)> {
    let temp = tempdir()?;
    let sh = Shell::new()?;
    cmd!(&sh, "cargo build --release --workspace").run()?;
    //Todo convert to xtask
    // Build the standard libs and copy them to the output directory
    cmd!(&sh, "./scripts/build.sh --release --package").run()?;
    // Copy the standard lib to the release target
    cmd!(&sh, "rm -rf target/release/stdlib").run()?;
    cmd!(&sh, "mv output target/release/stdlib").run()?;
    //Get rusty path
    let compile_dir = std::env::current_dir()?.join("target").join("release");
    let plc = std::env::current_dir()?.join("target").join("release").join("plc");
    if !plc.exists() {
        anyhow::bail!("Could not find compiler, did you run cargo build --release?")
    }
    //Export the standard lib location
    let lib_loc = compile_dir.join("stdlib");
    if !(lib_loc.exists()) {
        anyhow::bail!("Could not find stdlib, did you run the standard function compile script?")
    }
    std::env::set_var("STDLIBLOC", &lib_loc);
    Ok((temp, plc))
}
