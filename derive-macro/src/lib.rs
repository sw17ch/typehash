use quote::{quote, ToTokens};
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, Ident, Result, Type};

enum Input<'a> {
    Struct(Struct<'a>),
}

struct Struct<'a> {
    ident: &'a Ident,
    fields: Vec<StructField>,
}

struct StructField {
    ident: String,
    ty: Type,
}

fn struct_field<'a>(ix_field: (usize, &'a Field)) -> StructField {
    let (ix, field) = ix_field;
    let ty = field.ty.clone();
    StructField {
        ident: field
            .ident
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_else(|| ix.to_string()),
        ty,
    }
}

impl<'a> Struct<'a> {
    fn from_syn(ident: &'a Ident, data: &'a DataStruct) -> Struct<'a> {
        let fields: Vec<StructField> = data.fields.iter().enumerate().map(struct_field).collect();
        Struct { ident, fields }
    }
}

impl<'a> Input<'a> {
    fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            syn::Data::Struct(data) => Ok(Input::Struct(Struct::from_syn(&node.ident, data))),
            _ => panic!(),
        }
    }
}

fn check_type_impl(ty: &Type, seen: &mut Vec<Ident>) -> std::result::Result<(), String> {
    match ty {
        Type::Array(a) => check_type_impl(&a.elem, seen),
        Type::BareFn(_) => Err("barefn".into()),
        Type::Group(_) => Err("group".into()),
        Type::ImplTrait(_) => Err("impltrait".into()),
        Type::Infer(_) => Err("infer".into()),
        Type::Macro(_) => Err("macro".into()),
        Type::Never(_) => Err("never".into()),
        Type::Paren(_) => Err("paren".into()),
        Type::Path(p) => {
            if let Some(ident) = p.path.get_ident() {
                if seen.iter().find(|i| *i == ident).is_some() {
                    Err("recursive type detected".into())
                } else {
                    seen.push(ident.clone());
                    Ok(())
                }
            } else {
                let x = p.path.to_token_stream();
                let errstr = format!("type is not an ident: {}", x);
                Err(errstr)
            }
        }
        Type::Ptr(p) => check_type_impl(&p.elem, seen),
        Type::Reference(_) => Err("reference".into()),
        Type::Slice(_) => Err("slice".into()),
        Type::TraitObject(_) => Err("traitobject".into()),
        Type::Tuple(_) => Err("tuple".into()),
        Type::Verbatim(_) => Err("verbatim".into()),
        Type::__TestExhaustive(_) => panic!("test-exhaustive"),
    }
}

fn check_type(ident: &Ident, ty: &Type) -> std::result::Result<(), String> {
    let mut seen = vec![ident.clone()];
    check_type_impl(ty, &mut seen)
}

fn impl_struct(input: Struct) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let s = format!("struct {} {{ ", ident);
    let fields = input.fields.iter().map(|f| {
        let field_ident = &f.ident;
        let ty = &f.ty;
        if let Err(estr) = check_type(ident, ty) {
            let errstr = format!(
                "The type referred to by field '{}' is not supported: {}.",
                field_ident, estr
            );
            quote! {
                compile_error!(#errstr);
            }
        } else {
            quote! {
                s.push_str(#field_ident);
                s.push_str(": ");
                s.push_str(stringify!(#ty));
                s.push_str("=");
                s.push_str(&format!("{:016X}", &<#ty>::type_hash()));
                s.push_str(", ");
            }
        }
    });

    quote! {
        impl TypeHash for #ident {
            fn type_string() -> String {
                let mut s: String = #s.into();
                #(#fields)*
                s.push('}');
                s
            }
        }
    }
}

#[proc_macro_derive(TypeHash)]
pub fn derive_type_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = match Input::from_syn(&input) {
        Err(err) => return err.to_compile_error().into(),
        Ok(input) => input,
    };

    let output = match input {
        Input::Struct(i) => impl_struct(i),
    };

    proc_macro::TokenStream::from(output)
}
