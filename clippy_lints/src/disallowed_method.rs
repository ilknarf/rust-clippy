use crate::utils::span_lint;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir::*;
use rustc_span::Symbol;

declare_clippy_lint! {
    /// **What it does:** Lints for specific trait methods defined in clippy.toml
    ///
    /// **Why is this bad?** Some methods are undesirable in certain contexts,
    /// and it would be beneficial to lint for them as needed.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// foo.bad_method(); // trait Foo
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// GoodStruct::bad_method(); // no disallowed traits
    /// ```
    pub DISALLOWED_METHOD,
    nursery,
    "default lint description"
}

#[derive(Clone, Debug)]
pub struct DisallowedMethod {
    disallowed: FxHashMap<String, Vec<Vec<Symbol>>>,
}

impl DisallowedMethod {
    pub fn new(disallowed: FxHashSet<String>) -> Self {
        let mut disallowed_map: FxHashMap<String, Vec<Vec<Symbol>>> = FxHashMap::default();

        disallowed.iter().for_each(|method| {
            let mut v: Vec<_> = method.rsplit("::").collect();
            v.reverse();

                // ignore invalid inputs
                if v.len() > 1 {
                    let l = v.len() - 1;
                    let key = v[l].to_string();
                    let symbols: Vec<_> = v[..l].iter().map(|s| Symbol::intern(s)).collect();
                    // may be multiple traits with the same method name
                    if let Some(paths) = disallowed_map.get_mut(&key) {
                        paths.push(symbols);
                    }
                    else {
                        disallowed_map.insert(key.clone(), Vec::new());
                        let paths = disallowed_map.get_mut(&key).unwrap();
                        paths.push(symbols);
                    }
                }
        });

        Self { disallowed: disallowed_map }
    }
}

impl_lint_pass!(DisallowedMethod => [DISALLOWED_METHOD]);

impl <'tcx> LateLintPass<'tcx> for DisallowedMethod {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let ExprKind::MethodCall(path, _, _args, _) = &expr.kind {
            let method_name = path.ident.name.to_ident_string();
            if let Some(paths) = self.disallowed.get_mut(&method_name) {
                // get trt_id
                let def_id = cx.typeck_results().type_dependent_def_id(expr.hir_id).unwrap();
                let trt_id = cx.tcx.trait_of_item(def_id);

                // check possible paths
                paths.iter().for_each(|path| {
                    if trt_id.map_or(false, |trt_id| cx.match_def_path(trt_id, &path)) {
                        span_lint(
                            cx,
                            DISALLOWED_METHOD,
                            expr.span,
                            &format!(
                            "Use of a disallowed trait method `{}` of `{}`",
                            method_name,
                             path.iter().map(|s| s.to_ident_string()).collect::<Vec<_>>().join("::"))
                        );
                        return
                    }
                });
            }
        }
    }
}
