//! Test that we can compile a tuple struct.

use typehash::TypeHash;

#[derive(TypeHash)]
struct Tuple(usize, usize, u64);

fn main() {}