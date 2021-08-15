use typehash::{TypeHash, TypeString};

#[derive(TypeString)]
struct IndirectA {
    _ref: *const IndirectB,
}

#[derive(TypeString)]
struct IndirectB {
    _ref: *const IndirectA,
}

#[derive(TypeString)]
struct SelfRef {
    _ref: *const SelfRef,
}

fn main() {
    println!("{:016X}: {}", SelfRef::type_hash(), SelfRef::type_string());
    println!(
        "{:016X}: {}",
        IndirectA::type_hash(),
        IndirectA::type_string()
    );
    println!(
        "{:016X}: {}",
        IndirectB::type_hash(),
        IndirectB::type_string()
    );
}
