use proc_macro::TokenStream;

extern "C" {
    fn custom_to_string(v: i32) -> i32;
}

#[proc_macro_attribute]
pub fn some_name(a: TokenStream, _: TokenStream) -> TokenStream {
    unsafe {
        eprintln!("{:?}", custom_to_string(10));
    }
    a
}
