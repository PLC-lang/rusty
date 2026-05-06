use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use glob::glob;
use regex::Regex;

use crate::{
    build_config::{LinkageInfo, ProjectConfig},
    object::Object,
};

use plc::output::FormatOption;
use source_code::{SourceContainer, SourceType};

#[derive(Debug, Clone, Copy)]
pub enum Linkage {
    Static,
    Shared(Package),
}

/// How a library is intended to be packaged for the project
#[derive(Debug, Clone, Copy)]
pub enum Package {
    /// The library is available locally, it needs to be shipped with the project
    Local,
    /// The library is available on the target system, no need to ship it
    System,
}

/// Representation of a PLC Library
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

    pub fn get_compiled_lib(&self) -> &CompiledLibrary<T> {
        match &self.library {
            Library::Compiled(lib) => lib,
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
                let lib_path =
                    config.parent().map(|it| it.join(&conf.path)).unwrap_or_else(|| conf.path.clone());
                let linkage: Linkage = conf.package.into();

                let link_name = if let Some(link_path) = conf.link_path.as_ref() {
                    let resolved =
                        if link_path.is_absolute() { link_path.clone() } else { lib_path.join(link_path) };

                    if !resolved.is_file() {
                        return Err(anyhow!(
                            "configured link_path '{}' does not exist or is not a file",
                            resolved.display()
                        ));
                    }

                    resolved.to_string_lossy().to_string()
                } else {
                    conf.name.clone()
                };

                // Use the linkage type to find the library from the given name
                // TODO: We should allow for a fix name in the configuration if the library does not follow the unix convention
                // TODO: We should also allow a way to define objects based on the architecture
                let file_suffix_regex = match linkage {
                    Linkage::Static => Regex::new(r"\.a$").unwrap(),
                    Linkage::Shared(_) => Regex::new(r"\.so(\.\d+)*$").unwrap(),
                };

                let mut objects = vec![];
                for entry in std::fs::read_dir(&lib_path)?.flatten() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    if file_suffix_regex.is_match(&file_name_str) {
                        objects.push(entry.path().into());
                    }
                }

                let compiled_library = CompiledLibrary {
                    objects,
                    headers: resolve_file_paths(Some(&lib_path), conf.include_path)?,
                };

                Ok(LibraryInformation {
                    name: link_name,
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

    pub fn with_file_paths(self, files: Vec<PathBuf>) -> Result<Self> {
        let mut proj = self;
        let files = resolve_file_paths(proj.get_location(), files)?;
        for file in files {
            if matches!(file.get_type(), SourceType::Unknown) {
                let obj = file.into();
                proj.objects.push(obj);
            } else {
                proj.sources.push(file);
            }
        }
        Ok(proj)
    }

    pub fn with_include_paths(self, files: Vec<PathBuf>) -> Result<Self> {
        let mut proj = self;
        proj.includes = resolve_file_paths(proj.get_location(), files)?;
        Ok(proj)
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
}

fn resolve_file_paths(location: Option<&Path>, inputs: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut sources = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    //Ensure we are working with a directory
    let location = location.and_then(|it| if it.is_file() { it.parent() } else { Some(it) });
    for original_input in &inputs {
        let input = location.map(|it| it.join(original_input)).unwrap_or(original_input.to_path_buf());
        let path = &input.to_string_lossy();

        if let Err(e) = validate_input_exists(original_input, &input) {
            errors.push(e.to_string());
            continue;
        }

        let paths = match glob(path) {
            Ok(paths) => paths,
            Err(e) => {
                errors.push(format!("Failed to read glob pattern {path}: {e}"));
                continue;
            }
        };

        for p in paths {
            let resolved = p.context("Illegal Path").and_then(|p| p.canonicalize().context("Illegal Path"));
            match resolved {
                Ok(canonical) => sources.push(canonical),
                Err(e) => errors.push(e.to_string()),
            }
        }
    }
    if !errors.is_empty() {
        return Err(anyhow!("{}", errors.join("\n")));
    }
    Ok(sources)
}

/// Catches typos in project configurations by rejecting inputs that can never
/// match anything: literal paths whose target is missing, and globs whose
/// literal-prefix directory doesn't exist (e.g. `<typo>/include/*.st`).
/// Globs whose directory exists but match nothing remain accepted (a
/// templating pattern like `*.dt` is valid even when no .dt files are present),
/// regardless of whether the wildcard sits in the leaf segment or mid-path.
///
/// The wildcard probe is intentionally lax — a stray `[` without a matching
/// `]` (e.g. `[oddname.st`) classifies as a glob and routes through the
/// parent-directory check rather than being rejected as a literal miss. The
/// downstream `glob` crate would refuse the malformed pattern anyway; the
/// worst case is a slightly fuzzier error message.
///
/// Existence is determined via `Path::exists()`, which follows symlinks and
/// reports based on the target. A broken symlink rejects here; an unreadable
/// path that the caller lacks permission to traverse is reported as missing
/// only when the OS surfaces it that way (otherwise the failure resurfaces
/// at compile / link time with the underlying I/O error).
fn validate_input_exists(original: &Path, joined: &Path) -> Result<()> {
    let original_str = original.to_string_lossy();
    let has_wildcard = original_str.contains(['*', '?', '[']);

    if has_wildcard {
        let parent = glob_literal_parent(joined);
        if !parent.exists() {
            return Err(anyhow!("path '{original_str}' does not exist"));
        }
    } else if !joined.exists() {
        return Err(anyhow!("path '{original_str}' does not exist"));
    }
    Ok(())
}

/// Returns the deepest directory in `pattern` that contains no glob
/// metacharacter. For `src/foo/*.st` this is `src/foo`; for `*.st` it is `.`.
fn glob_literal_parent(pattern: &Path) -> PathBuf {
    let mut parent = PathBuf::new();
    for component in pattern.components() {
        let s = component.as_os_str().to_string_lossy();
        if s.contains(['*', '?', '[']) {
            break;
        }
        parent.push(component);
    }
    if parent.as_os_str().is_empty() {
        parent.push(".");
    }
    parent
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn literal_path_that_exists_resolves() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("main.st");
        File::create(&file).unwrap();

        let result = resolve_file_paths(Some(dir.path()), vec![PathBuf::from("main.st")]).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn literal_path_that_is_missing_errors() {
        let dir = tempdir().unwrap();

        let err = resolve_file_paths(Some(dir.path()), vec![PathBuf::from("missing.st")]).unwrap_err();
        assert!(err.to_string().contains("does not exist"), "got: {err}");
        assert!(err.to_string().contains("missing.st"), "got: {err}");
    }

    #[test]
    fn glob_with_existing_parent_and_no_matches_is_ok() {
        // A `*.dt` glob that matches nothing is a legitimate templating pattern
        // as long as the directory exists.
        let dir = tempdir().unwrap();

        let result = resolve_file_paths(Some(dir.path()), vec![PathBuf::from("*.dt")]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn glob_that_matches_files_resolves() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("a.st")).unwrap();
        File::create(dir.path().join("b.st")).unwrap();

        let result = resolve_file_paths(Some(dir.path()), vec![PathBuf::from("*.st")]).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn glob_with_missing_parent_directory_errors() {
        // The user's typical typo: `<typo>/include/*.st` where `<typo>` doesn't exist.
        // We still want to reject these even though the path contains a wildcard.
        let dir = tempdir().unwrap();

        let err =
            resolve_file_paths(Some(dir.path()), vec![PathBuf::from("missing-subdir/*.st")]).unwrap_err();
        assert!(err.to_string().contains("does not exist"), "got: {err}");
        assert!(err.to_string().contains("missing-subdir/*.st"), "got: {err}");
    }

    #[test]
    fn glob_literal_parent_strips_wildcard_components() {
        assert_eq!(glob_literal_parent(Path::new("src/foo/*.st")), PathBuf::from("src/foo"));
        assert_eq!(glob_literal_parent(Path::new("src/*/foo.st")), PathBuf::from("src"));
        assert_eq!(glob_literal_parent(Path::new("*.st")), PathBuf::from("."));
        // Defensive: production callers gate `glob_literal_parent` behind a
        // wildcard check, so a literal path never reaches here today. The
        // assertion documents the function's behaviour for that case in case
        // a future caller drops the gate.
        assert_eq!(glob_literal_parent(Path::new("src/file.st")), PathBuf::from("src/file.st"));
    }

    #[test]
    fn mid_segment_glob_with_existing_parent_and_no_matches_is_ok() {
        // `src/file*.st` is a glob even though the wildcard sits in the leaf
        // segment after a literal prefix. As long as the literal-prefix
        // directory exists (the parent), zero matches stays accepted — same
        // templating semantic as the simple `*.dt` case.
        let dir = tempdir().unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();

        let result = resolve_file_paths(Some(dir.path()), vec![PathBuf::from("src/file*.st")]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn project_config_with_files_resolving_empty_loads_with_empty_sources() {
        // A glob whose parent exists but matches nothing is allowed at this layer;
        // the driver layer enforces the "no input files" error so it covers both
        // build-config and CLI-args paths uniformly.
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("plc.json");
        std::fs::write(
            &config_path,
            r#"{ "name": "p", "files": ["*.dt"], "compile_type": "Static", "output": "out" }"#,
        )
        .unwrap();

        let project = Project::from_config(&config_path).unwrap();
        assert!(project.sources.is_empty());
    }

    #[test]
    fn project_config_with_files_present_resolves() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("main.st")).unwrap();
        let config_path = dir.path().join("plc.json");
        std::fs::write(
            &config_path,
            r#"{ "name": "p", "files": ["main.st"], "compile_type": "Static", "output": "out" }"#,
        )
        .unwrap();

        let project = Project::from_config(&config_path).unwrap();
        assert_eq!(project.sources.len(), 1);
    }
}
