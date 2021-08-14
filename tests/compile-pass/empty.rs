//! Test that we can compile an empty struct.

use typehash::TypeHash;

#[derive(TypeHash)]
struct Empty;

fn main() {}