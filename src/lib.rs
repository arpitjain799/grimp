use pyo3::prelude::*;
use std::collections::{HashSet, HashMap};

/// Return a tuple of modules in the shortest path between importer and imported.
//
//  If no path can be found, return None.
//
//  Args:
//    importer: the module doing the importing; the starting point.
//    imported: the module being imported: the end point.
//    importers_by_imported: Map of modules directly imported by each key.
//    importeds_by_importer: Map of all the modules that directly import each key.
#[pyfunction]
fn bidirectional_shortest_path(
    importer: String,
    imported: String,
    importers_by_imported: HashMap<String, HashSet<String>>,
    importeds_by_importer: HashMap<String, HashSet<String>>
) -> Option<Vec<String>> {
    let path = vec![importer, imported, "blue".to_string()];
    Some(path)
}

/// Performs a breadth first search from both source and target, meeting in the middle.
//
//  Returns:
//      (pred, succ, w) where
//         - pred is a dictionary of predecessors from w to the source, and
//         - succ is a dictionary of successors from w to the target.
//
fn search_for_path(
    importer: &String,
    imported: &String,
    importers_by_imported: HashMap<&String, HashSet<&String>>,
    importeds_by_importer: HashMap<&String, HashSet<&String>>
) -> Option<
        (
            HashMap<&String, Option<&String>>,
            HashMap<&String, Option<&String>>,
            &String
        )
     >
{
    if importer == imported {

        Some(
            (
                HashMap::from([
                    (&imported, None),
                ]),
                HashMap::from([
                    (&importer, None),
                ]),
                &importer
            )
        )
    }
    else {
        None // TODO
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn _grimp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bidirectional_shortest_path, m)?)?;
    Ok(())
}