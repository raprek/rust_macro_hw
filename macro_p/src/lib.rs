use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, LitStr, Token};

struct Input {
    values: Punctuated<LitStr, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(
            Self {
                values: Punctuated::parse_terminated(input)?
            }
        )
    }
}



#[proc_macro]
pub fn fn_calls(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as Input);
    let fn_names = input_parsed.values.iter()
        .filter(|x| { x.value().len() % 2  == 0 })
        .map(|x| {
        let fn_name: Expr = x.parse().unwrap();
        quote! {#fn_name(),}
    });
    
    quote! {{
        (
            #(
                #fn_names
            )*
        )

    }}
    .into()


}
