//! Self-referential types are not supported because the type-hash would depend
//! on itself. We reject these types at compile time.

use typehash::TypeString;

#[derive(TypeString)]
struct Indirect {
    _ref: *const SelfRef,
}

#[derive(TypeString)]
struct SelfRef {
    _ref: *const Indirect,
}

fn main() {}