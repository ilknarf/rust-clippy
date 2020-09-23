use crate::utils::{trait_ref_of_method, span_lint};

use rustc_data_structures::fx::FxHashSet;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir as hir;

declare_clippy_lint! {
    /// **What it does:** Lints for specific methods defined in clippy.toml
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
    /// foo.bad_method(); // type Foo
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// GoodStruct::bad_method();
    /// ```
    pub DISALLOWED_METHOD,
    nursery,
    "use of a disallowed method"
}

#[derive(Clone, Debug)]
pub struct DisallowedMethod {
    disallowed: FxHashSet<String>,
}

impl DisallowedMethod {
    pub fn new(disallowed: FxHashSet<String>) -> Self {
        Self { disallowed }
    }
}

impl_lint_pass!(DisallowedMethod => [DISALLOWED_METHOD]);

impl LateLintPass<'_> for DisallowedMethod {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ hir::Expr<'_>) {
        let p = trait_ref_of_method(cx, expr.hir_id).unwrap().path;
        let s = format!("{:?}", p);
        if !self.disallowed.contains(&s) {
            span_lint(
                cx,
                DISALLOWED_METHOD,
                expr.span,
                &format!("use of a disallowed method `{}`", s),
            )
        }
    }
}
