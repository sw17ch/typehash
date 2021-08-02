use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_empty() {
        #[derive(TypeHash)]
        struct Empty;
        assert_eq!("(struct Empty)", &Empty::type_string());
    }

    #[test]
    fn struct_tuple() {
        #[derive(TypeHash)]
        struct Tuple(usize, usize);
        assert_eq!("(struct Tuple (0) (1))", &Tuple::type_string());
    }

    mod o {
        use super::*;
        #[derive(TypeHash)]
        pub struct Other;
    }

    #[test]
    fn struct_fields() {
        #[derive(TypeHash)]
        struct Fields{ _a: usize, _b: usize, _c: o::Other }
        assert_eq!("(struct Fields (_a) (_b))", &Fields::type_string());
    }
}