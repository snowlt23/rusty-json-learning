
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_json_serialize(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let body = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => {
            let ident = body[0].ident.as_ref();
            let mut firstv = vec![quote! {
                s += "\"";
                s += stringify!(#ident);
                s += "\"";
                s += ":";
                s += &self.#ident.serialize();
            }];
            let mut restv = body[1..].iter()
            .filter_map(|field| field.ident.as_ref())
            .map(|ident| quote! {
                s += ",";
                s += "\"";
                s += stringify!(#ident);
                s += "\"";
                s += ":";
                s += &self.#ident.serialize();
            })
            .collect::<Vec<_>>();
            firstv.append(&mut restv);
            firstv
        },
        _ => panic!("ast is not struct!"),
    };
    quote! {
        impl Json for #name {
            fn serialize(&self) -> String {
                let mut s = "{".to_string();
                #(#body)*
                s += "}";
                s
            }
        }
    }
}

#[proc_macro_derive(JsonSerialize)]
pub fn json_serialize(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_json_serialize(&ast);
    gen.parse().unwrap()
}
