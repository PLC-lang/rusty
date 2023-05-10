use crate::traits::{Benchmark, Task};
use xshell::{cmd, Shell};

pub struct Oscat;
impl Task for Oscat {
    fn prepare(&self, sh: &Shell) -> anyhow::Result<()> {
        cmd!(sh, "git clone https://github.com/plc-lang/oscat ./benchmark/oscat").run()?;
        sh.create_dir("./benchmark/oscat/lib")?;
        sh.create_dir("./benchmark/oscat/include")?;

        cmd!(sh, "cargo b --release").run()?;
        sh.copy_file("./target/release/rustyc", "./benchmark/oscat")?;
        sh.copy_file("./target/release/libiec61131std.so", "./benchmark/oscat/lib")?;

        for file in sh.read_dir("libs/stdlib/iec61131-st")? {
            sh.copy_file(file, "./benchmark/oscat/include")?;
        }

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
