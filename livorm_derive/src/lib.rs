
extern crate proc_macro;

use quote::quote;
use syn;
use proc_macro::{TokenStream};


#[proc_macro_derive(HelloLiv)]
pub fn hello_liv_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_liv(&ast)
}


fn impl_hello_liv(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloLiv for #name {
            fn hello_liv() {
                println!("获取HelloLiv的宏开始执行");
                // if let Some(cli) = Self::get_client() {
                //     let db = (cli).database("liv");
                //     Ok(db)
                // } else {
                //     Err(())
                // }
            }
        }
    };
    gen.into()
}


