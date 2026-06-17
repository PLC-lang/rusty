use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use glob::{glob, Pattern};
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

        if let Err(e) = validate_input_exists(original_input, &input) {
            errors.push(e.to_string());
            continue;
        }

        // If the path exists literally on disk, resolve it directly instead of
        // treating it as a glob pattern. A folder or file name may legitimately
        // contain glob metacharacters (`[`, `]`, `?`) that are valid filename
        // characters on Windows and Linux, e.g. a directory named `[---test---]`.
        // Passing such a path to `glob()` would (mis)interpret the brackets as a
        // character class and silently match nothing.
        if input.exists() {
            match input.canonicalize().context("Illegal Path") {
                Ok(canonical) => sources.push(canonical),
                Err(e) => errors.push(e.to_string()),
            }
            continue;
        }

        // Otherwise treat `original_input` as a glob pattern. The `location`
        // prefix is always a literal directory path, so escape its glob
        // metacharacters and only let the input expand. This keeps globbing
        // (e.g. `files: ["*.st"]`) working when the project lives in a directory
        // whose name contains characters like `[` or `]`.
        let pattern = match location.filter(|_| original_input.is_relative()) {
            Some(loc) => format!(
                "{}{}{}",
                Pattern::escape(&loc.to_string_lossy()),
                std::path::MAIN_SEPARATOR,
                original_input.to_string_lossy()
            ),
            None => original_input.to_string_lossy().into_owned(),
        };

        let paths = match glob(&pattern) {
            Ok(paths) => paths,
            Err(e) => {
                errors.push(format!("Failed to read glob pattern {pattern}: {e}"));
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

/// Validates that input files actually exist before the compilation starts.
/// Ignores globs since they could point to a non existing (templating) directory.
fn validate_input_exists(original: &Path, joined: &Path) -> Result<()> {
    let original_str = original.to_string_lossy();
    let has_wildcard = original_str.contains(['*', '?', '[']);

    if !has_wildcard && !joined.exists() {
        return Err(anyhow!("path '{original_str}' does not exist"));
    }
    Ok(())
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
    fn glob_with_missing_parent_directory_resolves_empty() {
        // Optional directories are routine in shared project templates
        // (`conf/tasks/*.st` where `tasks/` doesn't exist in every project),
        // so a glob whose parent directory is missing simply matches nothing.
        // The driver still errors when the *entire* input set resolves empty.
        let dir = tempdir().unwrap();

        let result =
            resolve_file_paths(Some(dir.path()), vec![PathBuf::from("missing-subdir/*.st")]).unwrap();
        assert!(result.is_empty());
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

    // Regression tests for paths containing characters that are special in
    // glob syntax but are legitimate file/folder names on Windows and Linux.
    //
    // Windows forbids `< > : " / \ | ? *` and reserved names in path segments,
    // but permits `[ ] ( ) { } # + , ; = @ % $ ! ' ~ ^ &`, spaces, dots and
    // unicode. Of the permitted characters, `[` and `]` are glob metacharacters
    // (character classes/ranges), so historically a folder such as `[test]`
    // caused `glob()` to silently match nothing. These names are all valid on
    // both platforms and must round-trip through `resolve_file_paths`.
    //
    // See https://learn.microsoft.com/windows/win32/fileio/naming-a-file for the
    // Windows naming rules.

    /// Folder names that are legal on Windows *and* Linux. The ones containing
    /// `[`/`]` are the regression-critical cases (they look like glob patterns);
    /// the rest guard against any future change that starts mishandling other
    /// punctuation, spaces or unicode.
    const SPECIAL_FOLDER_NAMES: &[&str] = &[
        // The originally reported failure.
        "[---test---]",
        // Brackets that mimic glob character classes / ranges / negation.
        "[test]",
        "[abc]",
        "[a-z]",
        "[!abc]",
        "[^abc]",
        "[0-9]",
        // Unbalanced brackets — these would make `glob()` return a PatternError,
        // not just an empty match, yet they are perfectly valid folder names.
        "foo[bar",
        "foo]bar",
        "]",
        "[",
        // Brackets combined with other punctuation and spaces.
        "[build] (debug) #1",
        "src[v2]",
        // Braces (no brace-expansion in the `glob` crate, but still worth guarding).
        "{a,b}",
        "{release}",
        // Parentheses and common punctuation that is legal on both platforms.
        "project (v2)",
        "my project",
        "a+b",
        "a&b",
        "a#b",
        "a@b",
        "a%b",
        "a,b",
        "a;b",
        "a=b",
        "a~b",
        "a!b",
        "a$b",
        "a^b",
        "v1.2.3",
        "release-2026",
        // Unicode segments.
        "проект",
        "日本語",
        "café",
        "naïve",
    ];

    /// Resolving a literal file that lives inside a folder whose *name* contains
    /// glob metacharacters must find exactly that file. The folder name sits in
    /// the `location` prefix here, matching the bug report where the
    /// project directory (not the input string) carried the special characters.
    #[test]
    fn literal_file_inside_special_named_folder_resolves() {
        for name in SPECIAL_FOLDER_NAMES {
            let base = tempdir().unwrap();
            let dir = base.path().join(name);
            std::fs::create_dir(&dir).unwrap();
            File::create(dir.join("main.st")).unwrap();

            let result = resolve_file_paths(Some(&dir), vec![PathBuf::from("main.st")])
                .unwrap_or_else(|e| panic!("folder {name:?} failed to resolve: {e}"));

            assert_eq!(result.len(), 1, "folder {name:?} should resolve exactly one file");
            assert!(
                result[0].ends_with("main.st"),
                "folder {name:?} resolved unexpected path {:?}",
                result[0]
            );
        }
    }

    /// The special characters may also appear in the input string itself
    /// (e.g. `files: ["[---test---]/main.st"]` in a build description) rather
    /// than only in the location prefix.
    #[test]
    fn literal_input_path_with_special_characters_resolves() {
        for name in SPECIAL_FOLDER_NAMES {
            let base = tempdir().unwrap();
            let dir = base.path().join(name);
            std::fs::create_dir(&dir).unwrap();
            File::create(dir.join("main.st")).unwrap();

            let input = PathBuf::from(name).join("main.st");
            let result = resolve_file_paths(Some(base.path()), vec![input])
                .unwrap_or_else(|e| panic!("input under {name:?} failed to resolve: {e}"));

            assert_eq!(result.len(), 1, "input under {name:?} should resolve exactly one file");
        }
    }

    /// A file whose own name (not just its folder) contains glob metacharacters
    /// must resolve as a literal.
    #[test]
    fn literal_file_with_special_characters_in_filename_resolves() {
        let dir = tempdir().unwrap();
        let filenames = ["[main].st", "main[1].st", "a+b.st", "a&b.st", "v1.2.st"];
        for fname in filenames {
            File::create(dir.path().join(fname)).unwrap();
            let result = resolve_file_paths(Some(dir.path()), vec![PathBuf::from(fname)])
                .unwrap_or_else(|e| panic!("file {fname:?} failed to resolve: {e}"));
            assert_eq!(result.len(), 1, "file {fname:?} should resolve exactly one file");
        }
    }

    /// Genuine glob patterns must keep working even when the literal-prefix
    /// directory contains glob metacharacters in its name. The brackets in the
    /// folder name are literal (matched via the on-disk prefix), while the `*`
    /// in the leaf segment is expanded as a wildcard.
    #[test]
    fn glob_pattern_inside_special_named_folder_still_expands() {
        let base = tempdir().unwrap();
        let dir = base.path().join("[---test---]");
        std::fs::create_dir(&dir).unwrap();
        File::create(dir.join("a.st")).unwrap();
        File::create(dir.join("b.st")).unwrap();

        // `[---test---]` is a real directory, but the input itself does not
        // exist literally (it ends in `*.st`), so it goes through glob. The
        // bracketed prefix must still be treated as a literal directory.
        let result = resolve_file_paths(Some(&dir), vec![PathBuf::from("*.st")]).unwrap();
        assert_eq!(result.len(), 2, "glob inside a bracketed folder should match both files");
    }

    /// End-to-end through `Project::from_config`: a `plc.json` living inside a
    /// folder with glob metacharacters must load its sources.
    #[test]
    fn project_config_inside_special_named_folder_resolves() {
        for name in ["[---test---]", "[abc]", "foo[bar", "project (v2)", "日本語"] {
            let base = tempdir().unwrap();
            let dir = base.path().join(name);
            std::fs::create_dir(&dir).unwrap();
            File::create(dir.join("main.st")).unwrap();
            let config_path = dir.join("plc.json");
            std::fs::write(
                &config_path,
                r#"{ "name": "p", "files": ["main.st"], "compile_type": "Static", "output": "out" }"#,
            )
            .unwrap();

            let project = Project::from_config(&config_path)
                .unwrap_or_else(|e| panic!("project in folder {name:?} failed to load: {e}"));
            assert_eq!(project.sources.len(), 1, "project in folder {name:?} should have one source");
        }
    }

    /// A missing literal path inside a bracketed folder must still produce the
    /// "does not exist" error rather than being silently swallowed as an
    /// empty glob match.
    #[test]
    fn missing_literal_inside_special_named_folder_still_errors() {
        let base = tempdir().unwrap();
        let dir = base.path().join("[---test---]");
        std::fs::create_dir(&dir).unwrap();
        // Note: `missing.st` contains no glob metacharacters, so the absence is
        // reported; the bracketed *prefix* must not turn this into a glob.
        let err = resolve_file_paths(Some(&dir), vec![PathBuf::from("missing.st")]).unwrap_err();
        assert!(err.to_string().contains("does not exist"), "got: {err}");
    }
}
