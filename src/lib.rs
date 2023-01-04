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
fn bidirectional_shortest_path<'a>(
    importer: &'a str,
    imported: &'a str,
    importers_by_imported: &'a HashMap<&'a str, HashSet<&'a str>>,
    importeds_by_importer: &'a HashMap<&'a str, HashSet<&'a str>>
) -> Option<Vec<&'a str>> {
    let results_or_none = search_for_path(
        &importer,
        &imported,
        &importers_by_imported,
        &importeds_by_importer
    );
    match results_or_none {
        Some(results) => {

            let (pred, succ, initial_w) = results;

            let mut w_or_none: Option<&str> = Some(initial_w);
            // Transform results into tuple.
            let mut path: Vec<&str> = Vec::new();
            // From importer to w:
            while w_or_none.is_some() {
                let w = w_or_none.unwrap();
                path.push(w);
                w_or_none = pred[w];
            }
            path.reverse();

            // From w to imported:
            w_or_none = succ[path.last().unwrap()];
            while w_or_none.is_some() {
                let w = w_or_none.unwrap();
                path.push(w);
                w_or_none = succ[w];
            }

            Some(path)
        },
        None => None
    }

}

/// Performs a breadth first search from both source and target, meeting in the middle.
//
//  Returns:
//      (pred, succ, w) where
//         - pred is a dictionary of predecessors from w to the source, and
//         - succ is a dictionary of successors from w to the target.
//
fn search_for_path<'a>(
    importer: &'a str,
    imported: &'a str,
    importers_by_imported: &'a HashMap<&'a str, HashSet<&'a str>>,
    importeds_by_importer: &'a HashMap<&'a str, HashSet<&'a str>>
) -> Option<
        (
            HashMap<&'a str, Option<&'a str>>,
            HashMap<&'a str, Option<&'a str>>,
            &'a str
        )
     >
{
    if importer == imported {

        Some(
            (
                HashMap::from([
                    (imported, None),
                ]),
                HashMap::from([
                    (importer, None),
                ]),
                importer
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