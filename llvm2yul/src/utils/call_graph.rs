use std::collections::HashSet;

use llvm_ir_analysis::CallGraph;

use crate::utils::is_builtin;

fn _iter_functions(functions: &mut HashSet<String>, call_graph: &CallGraph<'_>, entry: &str) {
    let it = call_graph.callees(entry);
    for name in it {
        if !functions.contains(name) {
            functions.insert(name.into());

            if is_builtin(name) {
                log::debug!("Meet builtin function: {name}");
                continue;
            }

            _iter_functions(functions, call_graph, name)
        }
    }
}

pub fn get_all_callees(call_graph: &CallGraph<'_>, entry: &str) -> HashSet<String> {
    let mut functions = HashSet::new();

    _iter_functions(&mut functions, call_graph, entry);

    functions
}
