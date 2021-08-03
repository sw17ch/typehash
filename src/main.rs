use typehash::TypeHash;

#[derive(TypeHash)]
struct Empty;

#[derive(TypeHash)]
struct Tuple(usize, usize, u64);

#[derive(TypeHash)]
struct Fields {
    _empty: Empty,
    _usize_a: usize,
    _usize_b: usize,
    _tuple: Tuple,
    _array8: [usize; 8],
    _array9: [usize; 10],
}

#[derive(TypeHash)]
struct SelfRef {
    _ref: *const SelfRef,
}

fn main() {
    dbg!(Fields::type_string());
    dbg!(Fields::type_hash());
    dbg!(SelfRef::type_string());
}
