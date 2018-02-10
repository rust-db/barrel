//! A crate that handles some of the finer magical points of barrel error handling
//! 
//! The idea behind this is that most programmer errors will be caught when 
//! compiling the migrations, instead of during the runtime of a migration.
//! 
//! This means less hassle on your production system and better testability 🎉
//! 
//! More docs will follow here as the module matures


extern crate proc_macro;

extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("What am I already?? {}", stringify!(#name));
            }
        }
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}

// extern crate proc_macro;
// extern crate syn;

// #[macro_use]
// extern crate quote;

// use proc_macro::TokenStream;

// #[proc_macro_derive(Typed)]
// pub fn column_type_check(input: TokenStream) -> TokenStream {
//     let s = input.to_string();
//     let ast = syn::parse(input).unwrap();

//     panic!(s);

//     // unimplemented!();
// }   


// #[proc_macro_derive(HelloWorld)]
// pub fn hello_world(input: TokenStream) -> TokenStream {
//     // Construct a string representation of the type definition
//     let s = input.to_string();

//     // Parse the string representation
//     let ast = syn::parse_derive_input(&s).unwrap();

//     // Build the impl
//     let gen = impl_hello_world(&ast);
    
//     // Return the generated impl
//     gen.parse().unwrap()
// }

// fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
//     let name = &ast.ident;
//     quote! {
//         impl HelloWorld for #name {
//             fn hello_world() {
//                 println!("Hello, World! My name is {}", stringify!(#name));
//             }
//         }
//     }
// }