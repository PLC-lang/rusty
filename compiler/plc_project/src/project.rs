use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use glob::glob;

use crate::{
    build_config::{LinkageInfo, ProjectConfig},
    object::Object,
};

use plc::output::FormatOption;
use source_code::{SourceContainer, SourceType};

#[derive(Debug)]
pub enum Linkage {
    Static,
    Shared(Package),
}

/// How a library is intended to be packaged for the project
#[derive(Debug)]
pub enum Package {
    /// The library is available locally, it needs to be shipped with the project
    Local,
    /// The library is available on the target system, no need to ship it
    System,
}

/// Representation of a PLC Library
#[derive(Debug)]
pub enum Library<T: SourceContainer> {
    Compiled(CompiledLibrary<T>),
    Source(Project<T>),
}

/// A Compiled library to be included in the project
#[derive(Debug, Clone)]
pub struct CompiledLibrary<T: SourceContainer> {
    // TODO: name: String,
    //TODO: Version
    /// Location of the header files to be included in the project
    headers: Vec<T>,
    /// Objects files for the compiled library
    objects: Vec<Object>,
    // architectures: Vec<Target>,
}

/// The information required by a project to successfully include a library
#[derive(Debug)]
pub struct LibraryInformation<T: SourceContainer> {
    /// Location of the library if available
    location: Option<PathBuf>,
    /// Library name, this will be used when including the library
    name: String,
    /// How should the library be linked
    linkage: Linkage,
    /// The actual library in question
    library: Library<T>,
}

/// A PLC project to build
#[derive(Debug)]
pub struct Project<T: SourceContainer> {
    /// Name of the project
    name: String,
    /// The full path for the project, i.e where the build description exists
    location: Option<PathBuf>,
    //TODO: Version
    /// Source code for the project
    sources: Vec<T>,
    /// Files that will be referenced in the project but are not to be compiled (headers)
    includes: Vec<T>,
    /// Object files that do not need to be compiled
    objects: Vec<Object>,
    /// Libraries included in the project configuration
    libraries: Vec<LibraryInformation<T>>,
    /// Additional library paths to consider
    library_paths: Vec<PathBuf>,
    /// Output format
    format: FormatOption,
    /// Output Name
    output: Option<String>,
}

impl<T: SourceContainer> LibraryInformation<T> {
    pub fn get_includes(&self) -> &[T] {
        match &self.library {
            Library::Compiled(lib) => &lib.headers,
            Library::Source(lib) => lib.get_sources(),
        }
    }

    /// Returns the name used to link the Library
    pub fn get_link_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.location.as_deref()
    }

    pub fn should_copy(&self) -> bool {
        matches!(self.linkage, Linkage::Shared(Package::Local))
    }
}

impl<T: SourceContainer + Clone> LibraryInformation<T> {
    pub fn get_compiled_lib(&self) -> CompiledLibrary<T> {
        match &self.library {
            Library::Compiled(lib) => lib.clone(),
            _ => todo!("Convert source lib to compiled lib"),
        }
    }
}

impl<T: SourceContainer> CompiledLibrary<T> {
    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }
}

//configuration
impl Project<PathBuf> {
    /// Retrieve a project for compilation from a json description
    pub fn from_config(config: &Path) -> Result<Self> {
        let project_config = ProjectConfig::from_file(config)?;
        let libraries = project_config
            .libraries
            .into_iter()
            .map(|conf| {
                let lib_path = config.parent().map(|it| it.join(&conf.path)).unwrap_or_else(|| conf.path);
                let linkage: Linkage = conf.package.into();
                // Use the linkage type to find the library from the given name
                // TODO: We should allow for a fix name in the configuration if the library does not follow the unix convention
                // TODO: We should also allow a way to define objects based on the architecture
                let object_name = match linkage {
                    Linkage::Static => format! {"lib{}.a", &conf.name},
                    Linkage::Shared(_) => format! {"lib{}.so", &conf.name},
                };

                let lib_file = lib_path.join(object_name);
                let mut objects = vec![];
                if lib_file.exists() {
                    objects.push(lib_file.into());
                }
                let compiled_library = CompiledLibrary {
                    objects,
                    headers: resolve_file_paths(Some(&lib_path), conf.include_path)?,
                };
                Ok(LibraryInformation {
                    name: conf.name,
                    location: Some(lib_path),
                    linkage: conf.package.into(),
                    library: Library::Compiled(compiled_library),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let current_dir = env::current_dir()?;
        let location = config.parent().map(Path::to_path_buf).or(Some(current_dir));
        let sources = resolve_file_paths(location.as_deref(), project_config.files)?;
        Ok(Project {
            name: project_config.name,
            location,
            sources,
            libraries,
            format: project_config.compile_type,
            output: project_config.output,
            includes: vec![],
            objects: vec![],
            library_paths: vec![],
        })
    }

    pub fn with_file_paths(self, files: Vec<PathBuf>) -> Self {
        let mut proj = self;
        let files = resolve_file_paths(proj.get_location(), files).unwrap();
        for file in files {
            if matches!(file.get_type(), SourceType::Unknown) {
                let obj = file.into();
                proj.objects.push(obj);
            } else {
                proj.sources.push(file);
            }
        }
        proj
    }

    pub fn with_include_paths(self, files: Vec<PathBuf>) -> Self {
        let mut proj = self;
        proj.includes = resolve_file_paths(proj.get_location(), files).unwrap();
        proj
    }

}

impl<S: SourceContainer> Project<S> {
    pub fn new(name: String) -> Self {
        Project {
            name,
            location: None,
            sources: vec![],
            includes: vec![],
            objects: vec![],
            libraries: vec![],
            library_paths: vec![],
            format: FormatOption::default(),
            output: None,
        }
    }

    pub fn with_sources<T: IntoIterator<Item = S>>(mut self, sources: T) -> Self {
        self.sources.extend(sources);
        self
    }

    pub fn with_source_includes<T: IntoIterator<Item = S>>(mut self, includes: T) -> Self {
        self.includes.extend(includes);
        self
    }

    pub fn with_libraries(self, libraries: Vec<String>) -> Self {
        let mut proj = self;
        for library in libraries {
            proj.libraries.push(LibraryInformation {
                name: library.to_string(),
                location: None,
                linkage: Linkage::Shared(Package::System),
                library: Library::Compiled(CompiledLibrary { headers: vec![], objects: vec![] }),
            });
        }
        proj
    }

    pub fn with_library_paths(self, paths: Vec<PathBuf>) -> Self {
        let mut proj = self;
        proj.library_paths.extend(resolve_file_paths(proj.get_location(), paths).unwrap());
        proj
    }

    pub fn with_format(self, format: FormatOption) -> Self {
        let mut proj = self;
        proj.format = format;
        proj
    }

    pub fn with_output_name(self, output: Option<String>) -> Self {
        let mut proj = self;
        proj.output = output.or(proj.output);
        proj
    }

    pub fn get_library_paths(&self) -> &[PathBuf] {
        &self.library_paths
    }

    pub fn get_location(&self) -> Option<&Path> {
        self.location.as_deref()
    }

    pub fn get_sources(&self) -> &[S] {
        &self.sources
    }
    pub fn get_includes(&self) -> &[S] {
        &self.includes
    }

    pub fn get_libraries(&self) -> &[LibraryInformation<S>] {
        &self.libraries
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output_name(&self) -> String {
        self.output.as_ref().map(|it| it.to_string()).unwrap_or_else(|| {
            let input = self.get_name();
            match self.format {
                FormatOption::Object | FormatOption::Relocatable => format!("{input}.o"),
                FormatOption::Static => format!("{input}.out"),
                FormatOption::Shared | FormatOption::PIC | FormatOption::NoPIC => format!("{input}.so"),
                FormatOption::Bitcode => format!("{input}.bc"),
                FormatOption::IR => format!("{input}.ll"),
            }
        })
    }

    pub fn get_output_format(&self) -> FormatOption {
        self.format
    }

    /// Returns the validation schema used for this project
    pub fn get_validation_schema(&self) -> impl AsRef<str> {
        include_str!("../schema/plc-json.schema")
    }

    /// Returns the symbol name of this projects main initializer function
    pub fn get_init_symbol_name(&self) -> String {
        format!("__init___{}", self.get_name().replace('.', "_"))
    }
}

fn resolve_file_paths(location: Option<&Path>, inputs: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut sources = Vec::new();
    //Ensure we are working with a directory
    let location = location.and_then(|it| if it.is_file() { it.parent() } else { Some(it) });
    for input in &inputs {
        let input = location.map(|it| it.join(input)).unwrap_or(input.to_path_buf());
        let path = &input.to_string_lossy();
        let paths = glob(path).context(format!("Failed to read glob pattern {path}"))?;

        for p in paths {
            let path = p.context("Illegal Path")?;
            sources.push(path);
        }
    }
    Ok(sources)
}

impl From<LinkageInfo> for Linkage {
    fn from(value: LinkageInfo) -> Self {
        match value {
            LinkageInfo::Copy | LinkageInfo::Local => Self::Shared(Package::Local),
            LinkageInfo::System => Self::Shared(Package::System),
            LinkageInfo::Static => Self::Static,
        }
    }
}
