use quote::quote;
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

fn impl_struct(input: Struct) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let s = format!("struct {} {{", ident);
    let fields = input.fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            s.push_str(#ident);
            s.push_str(": ");
            s.push_str(stringify!(#ty));
            s.push_str("=");
            s.push_str(&format!("{:016X}", &<#ty>::type_hash()));
            s.push_str(", ");
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
