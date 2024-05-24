use std::env;

fn main() {
    //Initialize the logging
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if let Err(e) = plc_driver::compile(&args) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
