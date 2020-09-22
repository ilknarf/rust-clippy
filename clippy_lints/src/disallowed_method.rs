extern crate regex;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir::*;
use std::io;
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
    disallowed: FxHashMap<String, FxHashSet<String>>,
}

impl DisallowedMethod {
    pub fn new(disallowed: FxHashMap<String, FxHashSet<String>>) -> Self {
        Self { disallowed }
    }

    pub fn parse_disallowed_methods(blacklist: Vec<String>) -> FxHashMap<String, FxHashSet<String>> {
        let mut h = FxHashMap::default();
        let re = Regex::new(r"(.+)::(.*)");

        for method in blacklist {
            match re.captures_iter(*method).next().unwrap() {
                Some(caps) => {
                    let method_type = caps.get(0).unwrap();
                    let method_name = caps.get(1).unwrap();

                    let s = match h.get(method_type) {
                        Some(set) => set,
                        None => {
                            h.insert(method_type, FxHashSet::default());
                            h.get(method_type).unwrap()
                        },
                    };

                    s.insert(method_name);
                },
                None => (),
            }
        }

        h
    }
}

impl_lint_pass!(DisallowedMethod => [DISALLOWED_METHOD]);

impl LateLintPass<'_> for DisallowedMethod {}
