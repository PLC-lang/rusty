/// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder

use structopt::{StructOpt, clap::ArgGroup};

#[derive(StructOpt)]
#[structopt(
    group = ArgGroup::with_name("format").required(true),
    about = "IEC61131-3 Structured Text compiler powered by Rust & LLVM "
)]
pub struct CompileParameters {
    #[structopt(name = "input-file",
        help = "Read input from <input-file>")]
    pub input: String,

    #[structopt(short, long, name = "output-file", default_value="a.out",
        help = "Write output to <output-file>")]
    pub output: String,

    #[structopt(long="ir", group="format", 
        help = "Emit IR (LLVM Intermediate Representation) as output")]
    pub output_ir : bool,

    #[structopt(long="shared", group="format",
        help = "Emit a shared object as output")]
    pub output_shared_obj : bool,

    #[structopt(long="pic", group="format",
        help = "Emit PIC (Position Independent Code) as output")]
    pub output_pic_obj : bool,

    #[structopt(long="static", group="format",
        help = "Emit an object as output")]
    pub output_obj_code : bool,
    
    #[structopt(long="bc", group="format",
        help = "Emit binary IR (binary representation of LLVM-IR) as output")]
    pub output_bit_code : bool,

    #[structopt(long, name="<target-triple>",
        help = "A target-tripple supported by LLVM")]
    pub target : Option<String>,
}