use crate::compile::Compile;
use crate::run::Run;
use crate::task::Task;
use anyhow::Result;
use reporter::BenchmarkReport;
use xshell::{cmd, Shell};

mod compile;
mod reporter;
mod run;
mod task;

#[derive(Default)]
struct Parameters {
    action: Action,
    directory: Option<String>,
    reporter: ReporterType,
}

#[derive(Default)]
pub enum ReporterType {
    SQL,
    Git,
    #[default]
    Sysout,
}

#[derive(Default)]
enum Action {
    Run,
    Compile,
    #[default]
    All,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    prepare()?;
    let compiler = std::env::var("COMPILER")?;
    let params = parse_args(&args);
    //Create Reporter
    let reporter = reporter::from_type(params.reporter);
    //Create tasks
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    match &params.action {
        Action::Compile => {
            for opt in &["none", "less", "default", "aggressive"] {
                let task = Compile {
                    name: params.directory.as_ref().expect("Expected directory").to_string(),
                    directory: params.directory.as_ref().expect("Expected Directory").into(),
                    optimization: opt.to_string(),
                };
                tasks.push(Box::new(task));
            }
        }
        Action::Run => {
            for opt in &["none", "less", "default", "aggressive"] {
                let task = Run {
                    name: params.directory.as_ref().expect("Expected name").to_string(),
                    optimization: opt.to_string(),
                    compiler: compiler.clone(),
                    location: params.directory.as_ref().expect("Expected name").to_string(),
                    parameters: Some("--linker=cc".into()),
                };
                tasks.push(Box::new(task));
            }
        }
        Action::All => {
            //Create a default benchmark run
            //This includes oscat in 4 different opt
            for opt in &["none", "less", "default", "aggressive"] {
                let task = Compile {
                    name: "oscat".into(),
                    directory: "oscat".into(),
                    optimization: opt.to_string(),
                };
                tasks.push(Box::new(task));
            }

            // This includes the sieve of eratosthenes in
            // C
            for opt in &["0", "1", "2", "3"] {
                let task = Run {
                    name: "sieve-c".into(),
                    optimization: opt.to_string(),
                    compiler: "cc".into(),
                    location: "xtask/res/sieve.c".into(),
                    parameters: None,
                };
                tasks.push(Box::new(task));
            }
            // and ST
            for opt in &["none", "less", "default", "aggressive"] {
                let task = Run {
                    name: "sieve-st".into(),
                    optimization: opt.to_string(),
                    compiler: compiler.clone(),
                    location: "xtask/res/sieve.st".into(),
                    parameters: Some("--linker=cc".into()),
                };
                tasks.push(Box::new(task));
            }
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
    //Reprort data
    let report = BenchmarkReport::new(data)?;
    reporter.persist(report)?;
    Ok(())
}

fn parse_args(args: &[String]) -> Parameters {
    let mut params = Parameters::default();
    //Skip the name
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "compile" => params.action = Action::Compile,
            "run" => params.action = Action::Run,
            "all" => params.action = Action::All,
            _ => params.directory = Some(arg.to_string()),
        }
    }
    params
}

fn prepare() -> Result<()> {
    let sh = Shell::new()?;
    cmd!(&sh, "cargo build --release --workspace").run()?;
    //Todo convert to xtask
    // Build the standard libs and copy them to the output directory
    cmd!(&sh, "./libs/stdlib/scripts/build.sh --release --package").run()?;
    // Copy the standard lib to the release target
    cmd!(&sh, "rm -rf target/release/stdlib").run()?;
    cmd!(&sh, "mv output target/release/stdlib").run()?;
    //Get rusty path
    let compile_dir = std::env::current_dir()?.join("target").join("release");
    let plc = std::env::current_dir()?.join("target").join("release").join("rustyc");
    if !plc.exists() {
        anyhow::bail!("Could not find compiler, did you run cargo build --release?")
    }
    std::env::set_var("COMPILER", &plc);
    //Export the standard lib location
    let lib_loc = compile_dir.join("stdlib");
    if !(lib_loc.exists()) {
        anyhow::bail!("Could not find stdlib, did you run the standard function compile script?")
    }
    std::env::set_var("STDLIBLOC", &lib_loc);
    Ok(())
}
