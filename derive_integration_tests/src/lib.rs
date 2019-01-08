#![recursion_limit="126"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Ident};

use std::env;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::FromStr;

fn is_rust_kw(text: &AsRef<str>) -> bool {
    let tokens = TokenStream::from_str(text.as_ref());
    if tokens.is_err() { return false }

    // If we can't parse an ident out of this name then it's a keyword.
    match syn::parse::<Ident>(tokens.expect("Checked expect")) {
        Ok(_) => false,
        Err(_) => true
    }
}

fn make_ident(path: &AsRef<Path>) -> Ident {
    let mut path_string = path.as_ref().to_string_lossy()
        .replace("-", "_");

    if is_rust_kw(&path_string) {
        path_string.push('_');
    }

    Ident::new(&path_string, Span::call_site())
}

fn create_tests(path: &Path, mut path_name: Ident) -> quote::__rt::TokenStream {
    let children = fs::read_dir(path)
        .expect(&format!("Unable to read from {}", path.display()));
    let mut tests = Vec::new();
    for entry in children {
        let child_path = entry
            .expect(&format!("Unable to read from {}", path.display()))
            .path();
        if child_path.is_dir() {
            let child_name = make_ident(&child_path.strip_prefix(path)
                .expect("Child path was not a suffix of parent path"));
            tests.push(create_tests(&child_path, child_name));
        }
        else {
            if child_path.extension().map_or(true, |ex| &*ex != "protosnirk") {
                // Skip files that don't end in .protosnirk
                continue
            }

            let ignore = if path.components().any(|c| &c.as_os_str() == &"known-issues") {
                vec![quote! { #[ignore] }]
            }
            else {
                vec![]
            };

            let test_name = make_ident(&child_path.file_stem()
                .expect(&format!("No file stem on {}", child_path.display())));
            let child_path_string = child_path.to_string_lossy().to_string();
            tests.push(quote! {
                #[test]
                #(#ignore)*
                fn #test_name() {
                    use std::io::{Read, Write};
                    let mut buffer = String::new();
                    let mut file = ::std::fs::File::open(#child_path_string)
                        .expect(&format!("Unable to open {}",
                            #child_path_string));
                    file.read_to_string(&mut buffer)
                        .expect(&format!("Unable to read {}",
                            #child_path_string));
                    let test = crate::Test::new(&#child_path_string, buffer);
                    match crate::compile_runner(test) {
                        Ok(_) => {},
                        Err(reason) => panic!(reason)
                    }
                }
            });
        }
    }
    quote! {
        mod #path_name {
            #(#tests)*
        }
    }
}

#[proc_macro_derive(IntegrationTests)]
pub fn create_integration_tests(input: TokenStream) -> TokenStream {
    let full_path = env::current_dir()
        .expect("Can't `pwd`")
        .join("tests");

    let stream = create_tests(&full_path,
                              make_ident(&Path::new("tests")))
        .into();

    stream
}
