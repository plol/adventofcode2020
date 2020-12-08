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

fn parse_pattern(pattern: &String) -> Vec<&str> {
    let pattern_bytes = pattern.as_bytes();
    let mut i = 0;
    let mut result = vec![];
    let mut before = 0;
    while i < pattern_bytes.len() {
        if pattern_bytes[i] == b'{' {
            if pattern_bytes[i + 1] != b'}' {
                panic!();
            }
            if before != i {
                result.push(&pattern[before..i]);
            }
            result.push(&pattern[i..i + 2]);
            i += 2;
            before = i;
        } else {
            i += 1;
        }
    }
    if before != i {
        result.push(&pattern[before..]);
    }
    result
}

#[proc_macro]
pub fn scan_strs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ScanInput { pattern, to_parse } = syn::parse_macro_input!(input as ScanInput);
    let value = pattern.value();

    let mut parse_blobs = quote! {};

    let mut outputs = vec![];

    let parsed_pattern = parse_pattern(&value);
    for p in 0..parsed_pattern.len() {
        match parsed_pattern[p] {
            "{}" => {
                let output = format_ident!("output{}", format!("{}", p));
                outputs.push(output.clone());
                if p < parsed_pattern.len() - 1 {
                    let end = parsed_pattern[p + 1];
                    parse_blobs.extend(quote! {
                        let match_length = input[i..].find(#end).unwrap();
                        let #output = &(#to_parse)[i..i + match_length];
                        i += match_length;
                    });
                } else {
                    parse_blobs.extend(quote! {
                        let #output = &(#to_parse)[i..];
                    });
                }
            }
            s => {
                let len = s.len();
                parse_blobs.extend(quote! {
                    if i + #len <= input.len() && &input[i..i+#len] == #s {
                        i += #len;
                    } else {
                        panic!();
                    }
                });
            }
        }
    }

    (quote! {
        {
            let input = (#to_parse);

            let mut i = 0;
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
