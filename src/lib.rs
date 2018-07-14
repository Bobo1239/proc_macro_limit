#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

macro_rules! m {
    ($name:ident) => {
        #[proc_macro]
        pub fn $name(_: TokenStream) -> TokenStream {
            unimplemented!();
        }
    };
}

m!(m1);
m!(m2);
m!(m3);
m!(m4);
m!(m5);
m!(m6);
m!(m7);
m!(m8);
m!(m9);

m!(m10);
