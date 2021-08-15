//! Test that we can compile an empty struct.

use typehash::TypeString;

pub mod m {
    use super::*;
    #[derive(TypeString)]
    pub struct Other;
}

#[derive(TypeString)]
struct Empty {
    _other: m::Other,
}

fn main() {}