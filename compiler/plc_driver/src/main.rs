use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    //Initialize the logging
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    plc_driver::compile(&args)
}
