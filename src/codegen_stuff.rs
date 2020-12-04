extern crate proc_macro;

extern crate syn;
#[macro_use]
extern crate quote;

use quote::quote;

struct ScanInput {
    pattern: syn::LitStr,
    to_parse: syn::Expr,
}

impl syn::parse::Parse for ScanInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let pattern = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        Ok(ScanInput {
            pattern,
            to_parse: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn scan_strs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ScanInput { pattern, to_parse } = syn::parse_macro_input!(input as ScanInput);
    let value = pattern.value();
    let pattern_bytes = value.as_bytes();

    let mut parse_blobs = quote! {};

    let mut outputs = vec![];

    let mut i = 0;
    while i < pattern_bytes.len() {
        match pattern_bytes[i] {
            b'{' => {
                if pattern_bytes[i + 1] != b'}' {
                    panic!();
                }
                let output = format_ident!("output{}", format!("{}", i));
                outputs.push(output.clone());
                if i < pattern_bytes.len() - 2 {
                    let end = pattern_bytes[i + 2];
                    parse_blobs.extend(quote! {
                        match_start = i;
                        while input_bytes[i] != #end {
                            i += 1;
                        }
                        let #output = &(#to_parse)[match_start..i];
                    });
                } else {
                    parse_blobs.extend(quote! {
                        let #output = &(#to_parse)[i..];
                    });
                }
                i += 2;
            }
            b => {
                parse_blobs.extend(quote! {
                    if input_bytes[i] == #b {
                        i += 1;
                    } else {
                        panic!();
                    }
                });
                i += 1;
            }
        }
    }

    (quote! {
        {
            let mut i = 0;
            let input_bytes = (#to_parse).as_bytes();

            let mut i = 0;
            let mut match_start;
            #parse_blobs
            (#(#outputs),*)
        }
    })
    .into()
}

#[proc_macro]
pub fn scan(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ScanInput { pattern, to_parse } = syn::parse_macro_input!(input as ScanInput);

    let outputs = pattern
        .value()
        .as_bytes()
        .iter()
        .filter(|c| **c == b'{')
        .enumerate()
        .map(|(i, _)| format_ident!("output{}", format!("{}", i)))
        .collect::<Vec<_>>();

    let outputs_parsed = outputs.iter().map(|output| {
        quote! {
            #output.parse().unwrap()
        }
    });

    (quote! {
        {
            let (#(#outputs),*) = codegen_stuff::scan_strs!(#pattern, #to_parse);
            (#(#outputs_parsed),*)
        }
    })
    .into()
}
