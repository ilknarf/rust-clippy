#![warn(clippy::disallowed_method)]
#![allow(clippy::no_effect)]

struct Foo;

impl Foo {
    fn bad_method(self) {}
}

struct ImplStruct;

trait Baz {
    fn bad_method(self);
}

impl Baz for ImplStruct {
    fn bad_method(self) {}
}

struct NormalStruct;

impl NormalStruct {
    fn bad_method(self) {}
}

struct AttrStruct {
    bad_method: i32,
}

fn main() {
    let f = Foo;
    let b = ImplStruct;
    let n = NormalStruct;
    let a = AttrStruct{ bad_method: 5 };

    // lint these
    f.bad_method();
    b.bad_method();
    // these are good
    n.bad_method();
    a.bad_method;
}
