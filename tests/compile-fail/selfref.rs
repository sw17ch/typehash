use typehash::TypeHash;


#[derive(TypeHash)]
struct SelfRef {
    _ref: *const SelfRef,
}
