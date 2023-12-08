extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hello_macro_derive(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = func.sig;
    let func_name = &func_decl.ident; // function name
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            use std::time;
            println!("start 。。。");
            let start = time::Instant::now();
            #func_block
            println!("time cost {:?}", start.elapsed());
        }
    };
    caller.into()
}
