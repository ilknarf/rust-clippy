#![warn(clippy::blacklisted_method)]


struct Foo;

impl Foo {
    fn banned_method() {}
    fn good_method() {}
}

trait Baz {
    fn banned_method();
    fn good_method();
}

struct LegitStruct;

impl Baz for LegitStruct {
    fn banned_method() {}
    fn good_method() {}
}

struct UnbannedStruct;

impl UnbannedStruct {
    fn banned_method() {}
}

fn main() {
    let f = Foo;
    let b = Baz;
    let u = UnbannedStruct;
    // lints
    f.banned_method();
    b.banned_method();
    // good, method  not banned for type
    f.good_method();
    b.good_method();
    u.banned_method();
}
