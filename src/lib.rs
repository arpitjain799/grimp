use pyo3::prelude::*;
// use pyo3::types::PyUnicode;
use std::collections::{HashSet, HashMap};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn bidirectional_shortest_path(importer: String, imported: String, importers_by_imported: HashMap<String, HashSet<String>>, importeds_by_importer: HashMap<String, HashSet<String>>) -> Option<Vec<String>> {
    let path = vec![importer, imported, "blue".to_string()];
    Some(path)
}

/// A Python module implemented in Rust.
#[pymodule]
fn _grimp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bidirectional_shortest_path, m)?)?;
    Ok(())
}