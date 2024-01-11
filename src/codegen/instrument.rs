use inkwell::context::Context;
use rustc_llvm_coverage;
use std::ffi::CString;

pub struct InstrumentBuilder<'ink> {
    context: &'ink Context,
    files: Vec<&'ink str>,
}

impl<'ink> InstrumentBuilder<'ink> {
    pub fn new(context: &'ink Context, file_name: &'ink str) -> Self {
        Self { context, files: vec![file_name] }
    }

    pub fn write_header(&self) {
        // Filenames
        let cstring_filenames = self.files.iter().map(|f| CString::new(*f).unwrap()).collect::<Vec<_>>();
    }
}
