//! Test that we can compile a tuple struct.

use typehash::TypeString;

#[derive(TypeString)]
struct Tuple(usize, usize, u64);

fn main() {}