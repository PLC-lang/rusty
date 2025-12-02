use plc_ast::ast::CompilationUnit;

use crate::{
    header_generator::{
        file_helper::{get_header_file_information, FileHelper},
        header_generator_c::GeneratedHeaderForC,
    },
    GenerateHeaderOptions,
};

impl FileHelper for GeneratedHeaderForC {
    fn get_directory(&self) -> &str {
        &self.file_information.directory
    }

    fn get_path(&self) -> &str {
        &self.file_information.path
    }

    fn determine_header_file_information(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: &CompilationUnit,
    ) -> bool {
        let (file_information, determined_successfully) =
            get_header_file_information(generate_header_options, compilation_unit, "h");
        self.file_information = file_information;

        determined_successfully
    }
}
