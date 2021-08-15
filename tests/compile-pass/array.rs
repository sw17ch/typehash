//! Test that we can compile a struct holding a arrays.

use typehash::TypeString;

#[derive(TypeString)]
struct Fields {
    _array0: [usize; 0],
    _array1: [usize; 1],
    _array2: [usize; 2],
}

fn main() {}