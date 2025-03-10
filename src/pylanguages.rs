use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use tokei::Languages;

use crate::pyconfig::PyConfig;
use crate::pylanguage::{PyLanguage, ReportsPlain};
use crate::pylanguage_type::PyLanguageType;

#[pyclass(name = "Languages")]
pub struct PyLanguages {
    pub languages: Languages,
    pub ignored_files: Vec<String>
}

#[pymethods]
impl PyLanguages {
    #[new]
    pub fn new() -> Self {
        PyLanguages {
            languages: Languages::new(),
            ignored_files: Vec::new()
        }
    }
    #[getter]
    fn ignored_files(&self) -> Vec<String> {
        self.ignored_files.clone()  // Expose ignored files to Python
    }
    
    pub fn get_statistics(&mut self, paths: Vec<String>, mut ignored: Vec<String>, config: &PyConfig) -> PyResult<()> {
        if ignored.contains(&"all".to_string()) {
            Python::with_gil(|py| {
                // Get the package directory from Python
                let package = PyModule::import(py, "pytokei_new")?;
                let package_dir = package.getattr("__file__")?.to_string();
                
                // Construct the path to the ignore file
                let ignore_path = Path::new(&package_dir)
                    .parent()  // Move to pytokei_new directory
                    .expect("Failed to get package dir")
                    .join("data")  // Add data/
                    .join("ignore")  // Add ignore/
                    .join(".ignorerules.txt");  // Add .ignorerules.txt
    
                // Debug: Print the resolved ignore file path
                // println!("Resolved ignore file path: {:?}", ignore_path);
    
                if ignore_path.is_file() {
                    let file = File::open(&ignore_path)
                        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to open ignore file: {}", e)))?;
    
                    for line in io::BufReader::new(file).lines() {
                        let line = line.map_err(|e| PyErr::new::<PyValueError, _>(format!("Error reading line: {}", e)))?;
                        if !line.trim().is_empty() && !line.trim().starts_with('#') {
                            ignored.push(line);  // Add ignore patterns to the list
                        }
                    }
                } else {
                    println!("Ignore file not found at: {:?}", ignore_path);  // Debug: File missing
                }
                Ok::<(), PyErr>(())
            })?
        }
    
        let paths_: Vec<&str> = paths.iter().map(String::as_str).collect();
        let ignored_: Vec<&str> = ignored.iter().map(String::as_str).collect();
    
        // Debug: Print paths and ignore patterns
        // println!("Paths to scan: {:?}", paths_);
        // println!("Ignore patterns: {:?}", ignored_);
    
        self.languages
            .get_statistics(&paths_, &ignored_, &config.config);
    
        Ok(())
    }

    pub fn total(&self) -> PyLanguage {
        PyLanguage {
            language: self.languages.total(),
        }
    }

    pub fn language_names(&self) -> PyResult<Vec<&str>> {
        let vec = self
            .languages
            .iter()
            .map(|(lang_type, _)| lang_type.name())
            .collect();
        Ok(vec)
    }

    pub fn __getitem__(&self, lang_type: &PyLanguageType) -> Result<PyLanguage, PyErr> {
        let maybe_lang = self.languages.get(&lang_type.0);

        match maybe_lang {
            Some(maybe_lang) => Ok(PyLanguage {
                language: maybe_lang.clone(),
            }),
            None => Err(PyValueError::new_err(format!(
                "LanguageType not found: {}",
                lang_type.0
            ))),
        }
    }

    // Exposes the inner structure with the corresponding python classes
    pub fn get_languages(&self) -> HashMap<PyLanguageType, PyLanguage> {
        let map: HashMap<PyLanguageType, PyLanguage> = self
            .languages
            .iter()
            .map(|(x, y)| {
                (
                    PyLanguageType(x.clone()),
                    PyLanguage {
                        language: y.clone(),
                    },
                )
            })
            .collect();
        map
    }

    pub fn files(&self) -> HashMap<&str, usize> {
        let files = self
            .languages
            .iter()
            .map(|(lang_type, lang)| (lang_type.name(), lang.reports.len()))
            .collect();
        files
    }

    pub fn __repr__(&self) -> &str {
        return "Languages()";
    }

    pub fn get_languages_plain(&self) -> HashMap<&str, ReportsPlain> {
        // Corresponds to calling to the general command with --files and --compact
        let map: HashMap<&str, ReportsPlain> = self
            .languages
            .iter()
            .map(|(lang_type, lang)| {
                (
                    lang_type.name(),
                    PyLanguage {
                        language: lang.clone(),
                    }
                    .reports_plain(),
                )
            })
            .collect();
        map
    }

    pub fn total_plain(&self) -> HashMap<&str, usize> {
        // Returns the Total aggregation.
        let lang_total = self.languages.total();
        let map = HashMap::from([
            (
                "files",
                lang_total.children.values().map(Vec::len).sum::<usize>(),
            ),
            ("lines", lang_total.lines()),
            ("code", lang_total.code),
            ("comments", lang_total.comments),
            ("blanks", lang_total.blanks),
        ]);
        map
    }

    pub fn report_compact_plain(&self) -> HashMap<&str, HashMap<&str, usize>> {
        // Returns the info obtained from the default CLI command in compact mode
        let mut report = HashMap::new();

        for (ltype, lang) in &self.languages {
            let summary = lang.summarise();
            let stats = HashMap::from([
                ("lines", summary.lines()),
                ("code", summary.code),
                ("comments", summary.comments),
                ("blanks", summary.blanks),
                ("files", lang.reports.len())
            ]);
            report.insert(ltype.name(), stats);
        }
        report
    }
}
