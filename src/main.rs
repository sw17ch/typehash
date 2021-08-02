use typehash::TypeHash;

mod o {
    use super::*;
    #[derive(TypeHash)]
    pub struct Other;
}

#[derive(TypeHash)]
struct Tuple(usize, usize, u64);

#[derive(TypeHash)]
struct Fields {
    _a: usize,
    _b: usize,
    _c: o::Other,
    _t: Tuple,
}

fn main() {
    dbg!(Tuple::type_string());
    dbg!(Fields::type_string());
}
