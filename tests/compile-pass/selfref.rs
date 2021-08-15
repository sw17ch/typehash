//! Self-referential types are not supported because the type-hash would depend
//! on itself. We reject these types at compile time.

use typehash::TypeString;

#[derive(TypeString)]
struct SelfRef {
    _ref: *const SelfRef,
}

fn main() {}