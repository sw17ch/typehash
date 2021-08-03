use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub use typehash_macro::*;

pub trait TypeHash {
    fn type_string() -> String;
    fn type_hash() -> u64 {
        let mut hasher = DefaultHasher::new();
        Self::type_string().hash(&mut hasher);
        hasher.finish()
    }
}

impl TypeHash for usize {
    fn type_string() -> String {
        "usize".into()
    }
}

impl TypeHash for u64 {
    fn type_string() -> String {
        "u64".into()
    }
}

impl<T: TypeHash, const N: usize> TypeHash for [T; N] {
    fn type_string() -> String {
        format!("[{};{}]", T::type_hash(), N)
    }
}

impl<T: TypeHash> TypeHash for *const T {
    fn type_string() -> String {
        format!("*const {}", T::type_hash())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_empty() {
        #[derive(TypeHash)]
        struct Empty;
        assert_eq!("struct Empty { }", &Empty::type_string());
    }

    #[test]
    fn struct_tuple() {
        #[derive(TypeHash)]
        struct Tuple(usize, usize);
        let expected = format!(
            "struct Tuple {{ 0: usize={:016X}, 1: usize={:016X}, }}",
            usize::type_hash(),
            usize::type_hash()
        );
        assert_eq!(expected, Tuple::type_string());
    }

    #[test]
    fn struct_fields() {
        #[derive(TypeHash)]
        struct Fields {
            _a: usize,
            _b: usize,
        }
        let expected = format!(
            "struct Fields {{ _a: usize={:016X}, _b: usize={:016X}, }}",
            usize::type_hash(),
            usize::type_hash()
        );
        assert_eq!(expected, Fields::type_string());
    }

    #[test]
    fn struct_with_array() {
        #[derive(TypeHash)]
        struct Arrays {
            _a: [usize; 16],
            _b: [usize; 17],
        }
        let expected = format!(
            "struct Arrays {{ _a: [usize ; 16]={:016X}, _b: [usize ; 17]={:016X}, }}",
            <[usize; 16]>::type_hash(),
            <[usize; 17]>::type_hash()
        );
        assert_eq!(expected, Arrays::type_string());
    }
}
