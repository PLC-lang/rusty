use std::env;
fn main() {
    //Initialize the logging
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if let Err(err) = plc_driver::compile(&args) {
        err.exit()
    }
}
