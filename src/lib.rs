/*!
volatile_getset, we're ready to go!

A procedural macro for generating the most basic getters and setters on fields.

Getters are generated as `fn volatile_get_field(&self) -> type`, while setters are generated as `fn volatile_set_field(&mut self, val: type)`.

These macros are not intended to be used on fields which require custom logic inside of their setters and getters. Just write your own in that case!

```rust
#[macro_use]
extern crate volatile_getset;

#[derive(VolatileGetters, VolatileSetters, Default)]
pub struct Foo<T> where T: Copy + Clone + Default {
    /// Doc comments are supported!
    /// Multiline, even.
    #[volatile_get] #[volatile_set]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    #[volatile_get = "pub"] #[volatile_set = "pub"]
    public: T,
}

fn main() {
    let mut foo = Foo::default();
    foo.volatile_set_private(2);
    assert_eq!(foo.volatile_get_private(), 2);
}
```
*/


extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Field, DeriveInput};
use quote::Tokens;

mod volatile_getters;
mod volatile_setters;

#[proc_macro_derive(VolatileGetters, attributes(volatile_get))]
pub fn getters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for getters");

    // Build the impl
    let gen = produce(&ast, volatile_getters::implement);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(VolatileSetters, attributes(volatile_set))]
pub fn setters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Couldn't parse for setters");

    // Build the impl
    let gen = produce(&ast, volatile_setters::implement);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn produce(ast: &DeriveInput, worker: fn(&Field) -> Tokens) -> Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {

        let generated = fields.iter().map(worker).collect::<Vec<_>>();

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(VolatileGetters)] is only defined for structs, not for enums!");
    }
}