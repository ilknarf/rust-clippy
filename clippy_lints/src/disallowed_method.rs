extern crate regex;

use rustc_data_structures::fx::FxHashMap;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir as hir;
use if_chain::if_chain;
use regex::Regex;


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
    /// foo.bad_method();
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// goodstruct.bad_method();
    /// ```
    pub DISALLOWED_METHOD,
    nursery,
    "default lint description"
}

#[derive(Clone, Debug)]
pub struct DisallowedMethod {
    disallowed: FxHashMap<String, Vec<String>>,
}

impl DisallowedMethod {
    pub fn new(disallowed: FxHashMap<String, Vec<String>>) -> Self {
        Self { disallowed }
    }

    pub fn parse_disallowed_methods(blacklist: Vec<String>) -> FxHashMap<String, Vec<String>> {
        let mut h: FxHashMap<String, Vec<String>> = FxHashMap::default();
        let re = Regex::new(r"(.+)::(.*)").unwrap();

        for method in &blacklist {
            match re.captures_iter(method).next() {
                Some(caps) => {
                    let method_type = caps.get(0).unwrap().as_str().to_string();
                    let method_name = caps.get(1).unwrap().as_str().to_string();

                    let s = match h.get_mut(&method_type) {
                        Some(set) => set,
                        None => {
                            h.insert(method_type.clone(), Vec::new());
                            h.get_mut(&method_type).unwrap()
                        },
                    };

                    s.push(method_name);
                },
                None => (),
            }
        }

        h
    }
}

impl_lint_pass!(DisallowedMethod => [DISALLOWED_METHOD]);

impl LateLintPass<'_> for DisallowedMethod {
        fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ hir::Expr<'_>) {
        if_chain! {
            // Check our expr is calling a method
            if let hir::ExprKind::MethodCall(path, _, _args, _) = &expr.kind;
            // Check the name of this method is `some_method`
            if self.disallowed.contains_key(stringify!(path.ident.name));
            then {
                // ...
            }
        }
    }
}

