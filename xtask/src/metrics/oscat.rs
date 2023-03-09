use crate::traits::{Benchmark, Task};
use xshell::{cmd, Shell};

pub struct Oscat;
impl Task for Oscat {
    fn prepare(&self, sh: &Shell) -> anyhow::Result<()> {
        cmd!(sh, "git clone https://github.com/plc-lang/oscat ./benchmark/oscat").run()?;
        cmd!(sh, "git clone https://github.com/plc-lang/standardfunctions ./benchmark/oscat/sf").run()?;

        cmd!(sh, "cargo b --release").run()?;
        sh.copy_file("./target/release/rustyc", "./benchmark/oscat")?;

        sh.create_dir("./benchmark/oscat/lib").unwrap();
        sh.create_dir("./benchmark/oscat/include").unwrap();

        let standardfunctions = sh.push_dir("./benchmark/oscat/sf");
        cmd!(sh, "cargo b --release").run()?;
        std::mem::drop(standardfunctions);

        let oscat = sh.push_dir("./benchmark/oscat");
        sh.copy_file("sf/target/release/libiec61131std.so", "lib").unwrap();
        sh.read_dir("sf/iec61131-st/").unwrap().iter().for_each(|f| sh.copy_file(f, "include").unwrap());
        std::mem::drop(oscat);

        Ok(())
    }

    fn execute(&self, sh: &Shell, metrics: &mut super::Metrics) -> anyhow::Result<()> {
        let _oscat = sh.push_dir("./benchmark/oscat");

        for flag in ["none", "less", "default", "aggressive"] {
            cmd!(sh, "./rustyc -O{flag} build").ignore_stderr().benchmark(metrics, "oscat", flag)?;
        }

        cmd!(sh, "./rustyc check oscat.st")
            .ignore_status()
            .ignore_stderr()
            .benchmark(metrics, "check", "oscat")?;

        Ok(())
    }
}
