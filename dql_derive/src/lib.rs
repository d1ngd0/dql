use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField};
use proc_macro::{self, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Field, Ident, Type, parse_macro_input};

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(function))]
struct FunctionOpts {
    name: Option<String>,
}

#[derive(Default, FromField)]
#[darling(default, attributes(arg))]
struct FieldOpts {
    ignore: bool,
}

impl FunctionOpts {
    fn name_ident(&self, fallback: &Ident) -> Ident {
        let name = if let Some(name) = self.name.as_ref() {
            name.to_string()
        } else {
            fallback.to_string().to_case(Case::Snake)
        };
        syn::Ident::new(&name, Span::call_site().into())
    }
}

#[proc_macro_derive(Function, attributes(function))]
pub fn dql_function_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = FunctionOpts::from_derive_input(&input).expect("Wrong Options");
    dql_impl_function(input, opts)
}

fn dql_impl_function(ast: DeriveInput, opts: FunctionOpts) -> TokenStream {
    let function_name = opts.name_ident(&ast.ident);
    let name = &ast.ident;

    let Data::Struct(data) = ast.data else {
        unimplemented!()
    };

    let mut field_parse_logic = Vec::new();
    let mut field_ident = Vec::new();
    let mut should_parse_comma = false;
    for field in data.fields {
        if !should_parse_comma {
            should_parse_comma = true
        } else {
            field_parse_logic.push(quote! {
                crate::parser::consume_next!(self, crate::parser::FN_SEP)?;
            });
        }

        if let Some((ident, parse_logic)) = derive_parse_field(field) {
            field_parse_logic.push(parse_logic);
            field_ident.push(ident);
        }
    }

    let impl_gen = quote! {
        impl<'a> crate::Parser<'a> {
            fn #function_name(&self) -> crate::Result<#name> {
                crate::parser::consume_next!(self, "#function_name")?;
                crate::parser::consume_next!(self, crate::parser::FN_OPEN)?;
                #( #field_parse_logic )*
                crate::parser::consume_next!(self, crate::parser::FN_CLOSE)?;
                Ok(#name{
                    #( #field_ident ),*
                })
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#function_name{}{}", crate::parser::FN_OPEN, crate::parser::FN_CLOSE)
            }
        }
    };
    impl_gen.into()
}

fn derive_parse_field(field: Field) -> Option<(Ident, proc_macro2::TokenStream)> {
    let opts = FieldOpts::from_field(&field).expect("expected field options");
    impl_parse_field(field, opts)
}

fn impl_parse_field(field: Field, opts: FieldOpts) -> Option<(Ident, proc_macro2::TokenStream)> {
    if opts.ignore {
        return None;
    }
    let Type::Path(path) = field.ty else {
        unimplemented!();
    };

    let name = field.ident?.clone();

    Some((
        name.clone(),
        quote! {
            let #name: #path = TryFrom::try_from(self)?;
        },
    ))
}
