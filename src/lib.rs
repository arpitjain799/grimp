use pyo3::prelude::*;
use pyo3::types::{PyUnicode, PyDict, PyList};
use std::collections::{HashSet, HashMap};

#[pyfunction]
fn foo<'a>(
    mystring: &'a PyUnicode,
) -> PyResult<Vec<String>> {
    //let _mystring: String = mystring.extract()?;
    let mut list: Vec<String> = Vec::new();
    list.push(mystring.to_string());
    Ok(list)
}

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
    importer: &'a PyUnicode,
    imported: &'a PyUnicode,
    importers_by_imported: &'a PyDict,
    importeds_by_importer: &'a PyDict
) -> PyResult<Option<Vec<String>>> {
    let _importer: String = importer.extract()?;
    let _imported: String = imported.extract()?;
    let _importers_by_imported: HashMap<String, HashSet<String>> = importers_by_imported.extract()?;
    let _importeds_by_importer: HashMap<String, HashSet<String>> = importeds_by_importer.extract()?;
    Ok(None)
//     let path_or_none: Option<Vec<String>> = _bidirectional_shortest_path(
//         &_importer, &_imported, &_importers_by_imported, &_importeds_by_importer
//     );
//     Ok(path_or_none)
}


// fn _bidirectional_shortest_path<'a>(
//     importer: &'a str,
//     imported: &'a str,
//     importers_by_imported: &'a HashMap<String, HashSet<String>>,
//     importeds_by_importer: &'a HashMap<String, HashSet<String>>
// ) -> Option<Vec<String>> {
//     let results_or_none = search_for_path(
//         &importer,
//         &imported,
//         &importers_by_imported,
//         &importeds_by_importer
//     );
//     match results_or_none {
//         Some(results) => {
//
//             let (pred, succ, initial_w) = results;
//
//             let mut w_or_none: Option<String> = Some(initial_w.to_string());
//             // Transform results into tuple.
//             let mut path: Vec<String> = Vec::new();
//             // From importer to w:
//             while w_or_none.is_some() {
//                 let w = w_or_none.unwrap();
//                 path.push(w.to_string());
//                 w_or_none = pred[&w].as_ref();
//             }
//             path.reverse();
//
//             // From w to imported:
//             w_or_none = succ[path.last().unwrap()].as_ref();
//             while w_or_none.is_some() {
//                 let w = w_or_none.unwrap();
//                 path.push(w);
//                 w_or_none = succ[&w].as_ref();
//             }
//
//             Some(path)
//         },
//         None => None
//     }
// }
// /// Performs a breadth first search from both source and target, meeting in the middle.
// //
// //  Returns:
// //      (pred, succ, w) where
// //         - pred is a dictionary of predecessors from w to the source, and
// //         - succ is a dictionary of successors from w to the target.
// //
// fn search_for_path<'a>(
//     importer: &'a str,
//     imported: &'a str,
//     importers_by_imported: &'a HashMap<String, HashSet<String>>,
//     importeds_by_importer: &'a HashMap<String, HashSet<String>>
// ) -> Option<
//         (
//             HashMap<String, Option<String>>,
//             HashMap<String, Option<String>>,
//             &'a str
//         )
//      >
// {
//     if importer == imported {
//
//         Some(
//             (
//                 HashMap::from([
//                     (imported.to_string(), None),
//                 ]),
//                 HashMap::from([
//                     (importer.to_string(), None),
//                 ]),
//                 importer
//             )
//         )
//     }
//     else {
//         None // TODO
//     }
//
// }


/// A Python module implemented in Rust.
#[pymodule]
fn _grimp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bidirectional_shortest_path, m)?)?;
    m.add_function(wrap_pyfunction!(foo, m)?)?;
    Ok(())
}