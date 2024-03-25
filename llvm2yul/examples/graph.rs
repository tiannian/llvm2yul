use llvm_ir::Module;
use llvm_ir_analysis::ModuleAnalysis;

fn main() {
    let m = Module::from_ir_path("./llvm2yul/lls/test.ll").unwrap();
    let ma = ModuleAnalysis::new(&m);

    let fa = ma.fn_analysis("main");
    let ca = fa.control_dependence_graph();
}
