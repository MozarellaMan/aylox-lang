use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr, Token, bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token};

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
    let base_ident = syn::Ident::new(&base_name, base_name.span());
    let lines: Vec<String> = input.variants.into_iter().map(|s| s.value()).collect();
    
    let base = quote! {
        pub struct #base_ident;
    };

    output.extend(base.into_token_stream());
    
    for _type in lines.iter() {
        if _type.contains('/') {
            let seperated = _type.split('/').map(|s| s.trim()).collect::<Vec<_>>();
            let enum_name: &str = seperated[0];
            let variants: &str = seperated[1];
            output.extend(define_enum_type(&base_name, enum_name, variants));
        }
        
    }

   output
}

fn define_enum_type(base_name: &str, enum_name: &str, variants: &str) -> TokenStream {
    let variants: Vec<&str> = variants.split(',').map(|s| s.trim_start()).collect();

    let enum_name = Ident::new(&enum_name,enum_name.span());
    let mut names = Vec::new();
    let mut types = Vec::new();
    variants.iter().for_each(|v| {
        let split = v.split(' ').map(|s| s.trim()).collect::<Vec<_>>();
        dbg!(&split);
        names.push(Ident::new(split[0], split[0].span()));
        types.push(Ident::new(split[1], split[1].span()));
    });


    let new_enum = quote! {
        #[derive(Debug, Clone)]
        pub enum #enum_name {
            #(#names(#types))*,
        }
    };

    new_enum.into()
}