use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    bracketed, parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated,
    spanned::Spanned, token, Ident, LitStr, Token, Type,
};
struct Definition {
    name: LitStr,
    _first_comma: Token![,],
    _bracket_token: token::Bracket,
    variants: Punctuated<LitStr, Token![,]>,
}

impl Parse for Definition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let variants_content;
        Ok(Self {
            name: input.parse()?,
            _first_comma: input.parse()?,
            _bracket_token: bracketed!(variants_content in input),
            variants: Punctuated::parse_terminated(&variants_content)?,
        })
    }
}

#[proc_macro]
pub fn ast_gen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = transform_definition(parse_macro_input!(input as Definition));
    proc_macro::TokenStream::from(output)
}

fn transform_definition(input: Definition) -> TokenStream {
    let mut output = TokenStream::new();
    let base_name = input.name.value();
    let base_visitor_mutability = base_name.starts_with('~');
    let base_name = if base_visitor_mutability {
        base_name.strip_prefix('~').unwrap().to_string()
    } else {
        base_name
    };
    let base_name_plural: String = get_plural(&base_name);
    let base_name_plural = Ident::new(&base_name_plural, base_name_plural.span());
    let base_ident = Ident::new(&base_name, base_name.span());
    let base_ident_lowercase = Ident::new(
        &base_name.to_ascii_lowercase(),
        base_name.to_ascii_lowercase().span(),
    );
    let lines: Vec<String> = input.variants.into_iter().map(|s| s.value()).collect();
    let mut structs: Vec<Ident> = Vec::new();
    let mut non_base_types: Vec<Ident> = Vec::new();

    output.extend(quote! {
        pub type #base_name_plural<'a> = &'a [&'a #base_ident];
    });

    for _type in lines.iter() {
        if _type.contains('/') {
            let seperated = _type.split('/').map(|s| s.trim()).collect::<Vec<_>>();
            let enum_name = seperated[0];
            let variants = seperated[1];
            output.extend(define_enum_type(&base_name, enum_name, variants));
        } else if _type.contains(':') {
            let seperated = _type.split(':').map(|s| s.trim()).collect::<Vec<_>>();
            let struct_name = seperated[0];
            let fields = seperated[1];
            structs.push(Ident::new(struct_name, struct_name.span()));
            non_base_types.push(Ident::new(
                &struct_name.to_ascii_lowercase(),
                struct_name.to_ascii_lowercase().span(),
            ));
            output.extend(define_struct_type(&base_name, struct_name, fields));
        } else {
            let new_struct = _type.trim();
            let new_struct = Ident::new(new_struct, new_struct.span());
            output.extend(quote! {
                #[derive(Debug, Copy, Clone, Eq, PartialEq, new)]
                pub struct #new_struct;
            });
        }
    }

    let visitor_names: Vec<Ident> = non_base_types
        .iter()
        .map(|ident| format_ident!("visit_{}", ident))
        .collect();
    let base_visitor_name = format_ident!("visit_{}", &base_ident_lowercase);
    let base_visitor_trait_name = format_ident!("{}Visitor", &base_ident);
    let base = quote! {
        #[derive(Debug, Clone, new)]
        pub enum #base_ident {
            #(#structs(#structs)),*
        }
    };
    let visitor = if base_visitor_mutability {
        quote! {
            pub trait #base_visitor_trait_name<T> {
                #(fn #visitor_names(&mut self, #non_base_types: &#structs) -> T);*;

                fn #base_visitor_name(&mut self, #base_ident_lowercase: &#base_ident) -> T {
                    match &#base_ident_lowercase {
                        #(#base_ident::#structs(val) => self.#visitor_names(val)),*
                    }
                }
            }
        }
    } else {
        quote! {
            pub trait #base_visitor_trait_name<T> {
                #(fn #visitor_names(&self, #non_base_types: &#structs) -> T);*;

                fn #base_visitor_name(&self, #base_ident_lowercase: &#base_ident) -> T {
                    match &#base_ident_lowercase {
                        #(#base_ident::#structs(val) => self.#visitor_names(val)),*
                    }
                }
            }
        }
    };
    output.extend(base);
    output.extend(visitor);
    output
}

fn get_plural(base_name: &str) -> String {
    if base_name.ends_with("pr") {
        format!("{}essions", base_name)
    } else {
        format!("{}s", base_name)
    }
}

fn define_struct_type(base_name: &str, struct_name: &str, fields: &str) -> TokenStream {
    let fields: Vec<&str> = fields.split(',').map(|s| s.trim_start()).collect();
    let struct_name = Ident::new(&struct_name, struct_name.span());
    let mut names: Vec<Ident> = Vec::new();
    let mut types: Vec<Type> = Vec::new();

    fields.iter().for_each(|v| {
        generate_types_names(v, base_name, &mut types, &mut names);
    });

    let new_struct = quote! {
        #[derive(Debug, Clone, new)]
        pub struct #struct_name {
            #(pub #names:#types),*
        }
    };

    new_struct
}

fn define_enum_type(base_name: &str, enum_name: &str, variants: &str) -> TokenStream {
    let variants: Vec<&str> = variants.split(',').map(|s| s.trim_start()).collect();

    let enum_name = Ident::new(&enum_name, enum_name.span());
    let mut names = Vec::new();
    let mut types = Vec::new();
    variants.iter().for_each(|v| {
        generate_types_names(v, base_name, &mut types, &mut names);
    });

    let new_enum = quote! {
        #[derive(Debug, Clone, new, PartialEq)]
        pub enum #enum_name {
            #(#names(#types)),*
        }
    };

    new_enum
}

fn generate_types_names(v: &&str, base_name: &str, types: &mut Vec<Type>, names: &mut Vec<Ident>) {
    let split = v.split(' ').map(|s| s.trim()).collect::<Vec<_>>();
    let _type = split[0];
    let name = split[1];
    let type_optional = _type.ends_with('?');
    let type_ident = if type_optional {
        let _type = _type.strip_suffix('?').unwrap();
        Ident::new(_type, _type.span())
    } else {
        Ident::new(_type, _type.span())
    };
    let _type = if _type == base_name {
        parse_quote!(Box<#type_ident>)
    } else {
        parse_quote!(#type_ident)
    };

    let _type = if type_optional {
        parse_quote!(Option<#_type>)
    } else {
        _type
    };

    names.push(Ident::new(name, name.span()));
    types.push(_type);
}
