#![warn(clippy::disallowed_method)]

struct Foo;

impl Foo {
    fn bad_method() {}
}

struct ImplStruct;

trait Baz{
    fn bad_method();
}

impl Baz for ImplStruct {
    fn bad_method() {}
}

struct NormalStruct;

impl NormalStruct {
    fn bad_method() {}
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
