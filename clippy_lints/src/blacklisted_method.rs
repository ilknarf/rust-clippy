extern crate regex;
use regex::Regex;
use rustc_data_structures::fx::FxHashMap;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir::*;

declare_clippy_lint! {
    /// **Wh4jat it does:** Checks for usage of blacklisted methods, such as `Foo::bar`
    ///
    /// **Why is this bad?** Some operations and functions are unsafe and should be
    /// avoided, e.g. unsafe arithmetic.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// Foo.method_name();
    /// ```
    /// Use instead:
    /// ```rust
    /// SomeStruct.method_name();
    /// ```
    pub BLACKLISTED_METHOD,
    nursery,
    "usage of a blacklisted method"
}

#[derive(Clone, Debug)]
pub struct BlacklistedMethod {
    blacklist: FxHashMap<String, String>,
}

impl BlacklistedMethod {
    pub fn new(blacklist: FxHashMap<String, String>) -> Self {
        Self { blacklist }
    }

    pub fn parse_blacklist(raw_blacklist: Vec<String>) -> FxHashMap<String, String> {
        let h: FxHashMap<String, String> = FxHashMap::default();
        // simplified regex parser
        let re = Regex::new(r"(.+)::[^:]+").unwrap();

        for s in raw_blacklist.iter() {

        }

        h
    }
}

impl_lint_pass!(BlacklistedMethod => [BLACKLISTED_METHOD]);

impl <'tcx> LateLintPass<'tcx> for BlacklistedMethod {
    fn check_pat(&mut self, cx: &LateContext<'tcx>, pat: &'tcx Pat<'_>) {

    }
}
