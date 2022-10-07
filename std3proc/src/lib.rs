#![no_std]

extern crate alloc;
extern crate proc_macro;
use proc_macro as pm;
use alloc::{
    string::{
        ToString,
        String,
    },
    format
};


#[proc_macro_attribute]
pub fn main(_args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let mut fn_body: String = String::new();
    let mut inputiter = input.into_iter();
    let fn_name = match &mut inputiter.nth(1) {
        Some(v) => v.to_string(),
        None => String::from("__fnname")
    };
    for i in &mut inputiter {
        fn_body += &i.to_string();
    };
    let x = format!("kernel!({fname});\nfn {fname}{fbody}",fname=fn_name,fbody=fn_body);
    x.parse().expect("Generated invalid tokens")
}