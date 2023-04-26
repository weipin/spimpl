// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements the derive macros for the trait `rlp::Encode` and `rlp::Decode`.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Ident,
};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let generics = add_trait_encode_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encoding = encode_struct_fields(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics rlp::Encode for &#name #ty_generics #where_clause {
            #[allow(clippy::needless_borrow)]
            fn encode_to(self, output: &mut Vec<u8>) {
                #encoding
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let generics = add_trait_decode_bounds(input.generics);
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let item_type = decoding_struct_item_type(&input.data);
    let decoding = decode_struct_fields(&name, &input.data);

    let expanded = quote! {

        // The generated impl.
        impl<'a> rlp::Decode<'a> for #name #ty_generics #where_clause {
            const TYPE: rlp::ItemType = #item_type;

            fn decode(payload: rlp::ItemPayloadSlice<'a>) -> Result<Self, rlp::Error> {
                #decoding
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: Encode` to every type parameter T.
fn add_trait_encode_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(rlp::Encode));
        }
    }
    generics
}

// Generate expressions to encode the fields and append the result to `output`.
fn encode_struct_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to expressions like
                    //
                    // ```
                    // encode_to(&self.x, &mut payload);
                    // encode_to(&self.y, &mut payload);
                    // encode_to(&self.z, &mut payload);
                    // ```
                    //
                    // but using fully qualified function call syntax.
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            rlp::encode_to(&self.#name, &mut payload);
                        }
                    });
                    quote! {
                        let mut payload = vec![];
                        #(#recurse)*

                        rlp::ItemPayloadSlice(&payload).encode_as_list(output);
                    }
                }
                Fields::Unnamed(ref fields) => {
                    if fields.unnamed.len() == 1 {
                        // For "newtype", encode as a single value
                        let field = fields.unnamed.first().unwrap();
                        quote_spanned! {field.span()=>
                            rlp::encode_to(&self.0, output);
                        }
                    } else {
                        // tuple struct not supported
                        unimplemented!();
                    }
                }
                Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Add a bound `T: Decode` to every type parameter T.
fn add_trait_decode_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(rlp::Decode));
        }
    }
    generics
}

// Generate expressions to decode the fields and create a new `#name`.
fn decode_struct_fields(name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to expressions like
                    //
                    // ```
                    // let a: TYPE_A = list_iter.next_item()?;
                    // let b: TYPE_B = list_iter.next_item()?;
                    // let c: TYPE_C = list_iter.next_item()?;
                    // ```
                    let recurse1 = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            let #name: #ty = list_iter.next_item()?;
                        }
                    });

                    // Expands to a expression like
                    //
                    // ```
                    // a, b, c
                    // ```
                    let recurse2 = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! {
                            #name
                        }
                    });
                    quote! {
                        let mut list_iter = payload.list_iter_unchecked();
                        #(#recurse1)*

                        if !list_iter.next().is_none() {
                            return Err(rlp::Error::ListDecodingNumberDoesNotMatch);
                        }

                        Ok(#name { #(#recurse2),* })
                    }
                }
                Fields::Unnamed(ref fields) => {
                    if fields.unnamed.len() == 1 {
                        // Decode "newtype" as a single value
                        let field = fields.unnamed.first().unwrap();
                        let ty = &field.ty;
                        quote! {
                            let value = <#ty as rlp::Decode>::decode(payload)?;
                            Ok(#name (value))
                        }
                    } else {
                        // tuple struct not supported
                        unimplemented!();
                    }
                }
                Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate item type
fn decoding_struct_item_type(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(_) => {
                quote! {
                    rlp::ItemType::List
                }
            }
            Fields::Unnamed(ref fields) => {
                if fields.unnamed.len() == 1 {
                    // "newtype" should be represented as a single value
                    quote! {
                        rlp::ItemType::SingleValue
                    }
                } else {
                    // tuple struct not supported
                    unimplemented!();
                }
            }
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
