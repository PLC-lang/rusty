use plc_ast::ast::CompilationUnit;

use crate::{
    header_generator::{
        file_helper::{format_file_name, get_header_file_information, FileHelper},
        header_generator_c::GeneratedHeaderForC,
    },
    GenerateHeaderOptions,
};

impl FileHelper for GeneratedHeaderForC {
    fn get_directory(&self) -> &str {
        &self.file_information.directory
    }

    fn set_directory(&mut self, directory: &str) {
        self.file_information.directory = directory.to_string();
    }

    fn get_path(&self) -> &str {
        &self.file_information.path
    }

    fn set_path(&mut self, path: &str) {
        self.file_information.path = format!("{path}.h");
    }

    fn set_file_name(&mut self, file_name: &str) {
        self.file_information.name = format_file_name(file_name);
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
