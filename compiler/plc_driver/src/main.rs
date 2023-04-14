use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args: Vec<String> = std::env::args().collect();
    plc_driver::compile(&args).unwrap();
}
