use rustc_data_structures::fx::FxHashMap;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir::*;

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
    /// Foo.bad_method();
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// GoodStruct.bad_method();
    /// ```
    pub DISALLOWED_METHOD,
    nursery,
    "default lint description"
}

#[derive(Clone, Debug)]
pub struct DisallowedMethod {
    disallowed: FxHashMap<String, String>,
}

impl DisallowedMethod {
    pub fn new(disallowed: FxHashMap<String, String>) -> Self {
        Self { disallowed }
    }

    pub fn parse_disallowed_methods(blacklist: Vec<String>) -> io::Result<FxHashMap<String, String>> {
        let mut h = FxHashMap::default();

        for method in blacklist {

        }
    }
}

impl_lint_pass!(DisallowedMethod => [DISALLOWED_METHOD]);

impl LateLintPass<'_> for DisallowedMethod {}
