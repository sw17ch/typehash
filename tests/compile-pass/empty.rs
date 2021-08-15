//! Test that we can compile an empty struct.

use typehash::TypeString;

#[derive(TypeString)]
struct Empty;

fn main() {}