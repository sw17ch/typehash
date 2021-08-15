use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub use typehash_macro::*;

pub trait TypeHash {
    fn type_hash() -> u64;
}

impl<T: TypeString> TypeHash for T {
    fn type_hash() -> u64 {
        let mut hasher = DefaultHasher::new();
        Self::type_string().hash(&mut hasher);
        hasher.finish()
    }
}

pub trait TypeString {
    fn type_name() -> &'static str;
    fn type_string_impl(seen: &mut Vec<&'static str>) -> String;
    fn type_string() -> String {
        let mut seen = Vec::new();
        Self::type_string_impl(&mut seen)
    }
}

impl TypeString for usize {
    fn type_name() -> &'static str {
        "usize"
    }
    fn type_string_impl(_seen: &mut Vec<&'static str>) -> String {
        Self::type_name().into()
    }
}

impl TypeString for u64 {
    fn type_name() -> &'static str {
        "u64"
    }
    fn type_string_impl(_seen: &mut Vec<&'static str>) -> String {
        Self::type_name().into()
    }
}

impl<T: TypeString, const N: usize> TypeString for [T; N] {
    fn type_name() -> &'static str {
        T::type_name()
    }
    fn type_string_impl(seen: &mut Vec<&'static str>) -> String {
        format!("[{};{}]", T::type_string_impl(seen), N)
    }
}

impl<T: TypeString> TypeString for *const T {
    fn type_name() -> &'static str {
        T::type_name()
    }
    fn type_string_impl(seen: &mut Vec<&'static str>) -> String {
        format!("*const {}", T::type_string_impl(seen))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_empty() {
        #[derive(TypeString)]
        struct Empty;
        assert_eq!("struct Empty { }", &Empty::type_string());
    }

    #[test]
    fn struct_tuple() {
        #[derive(TypeString)]
        struct Tuple(usize, usize);
        let expected = "struct Tuple { 0: <usize>, 1: <usize>, }";
        assert_eq!(expected, Tuple::type_string());
    }

    #[test]
    fn struct_fields() {
        #[derive(TypeString)]
        struct Fields {
            _a: usize,
            _b: usize,
        }
        let expected = "struct Fields { _a: <usize>, _b: <usize>, }";
        assert_eq!(expected, Fields::type_string());
    }

    #[test]
    fn struct_with_array() {
        #[derive(TypeString)]
        struct Arrays {
            _a: [usize; 16],
            _b: [usize; 17],
        }
        let expected = "struct Arrays { _a: <[usize;16]>, _b: <[usize;17]>, }";
        assert_eq!(expected, Arrays::type_string());
    }
}
