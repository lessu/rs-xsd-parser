extern crate proc_macro;
use quote::quote;
use proc_macro::TokenStream;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn string_based(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let struct_impl = match &mut input.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { pub str: String }).unwrap());
                }   
                _ => { () }
            }
            quote! { #input }
        }
        _ => panic!("`add_field` has to be used with structs "),
    };
    // Generate the implementation for YaDeserialize and YaSerialize
    let expanded = quote! {
        use std::fmt::Write;
        #struct_impl
        impl #name{
            pub fn as_str(&self) -> &str{
                self.str.as_str()
            }
        }
        impl yaserde::YaDeserialize for #name {
            fn deserialize<R: std::io::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
                let mut ret = String::new();
                let mut err = None;
                let mut level:u32 = 0;
                loop {
                    let event = reader.peek()?;
                    match event {
                        xml::reader::XmlEvent::StartElement { name:_, attributes:_, namespace:_ } => {
                            if level == 0 {
                                level += 1;
                            }else{
                                err = Some("#name should be a text, but get a complex type".to_string());
                                break;
                            }
                        }
                        xml::reader::XmlEvent::EndElement { name: _ } => {
                            level = level - 1;
                            if level == 0 {
                                break;
                            }
                        }
                        xml::reader::XmlEvent::Characters(ref text_content) => {
                            ret.write_str(text_content).unwrap();
                        }
                        _ => {
                            // pass
                        }
                    }
                    reader.next_event()?;
                }
                if let Some(err_str) = err {
                    Err(err_str)
                } else {
                    Ok(#name { str: ret })
                }
            }
        }

        impl yaserde::YaSerialize for #name {
            fn serialize<W: std::io::Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
                writer.write(yaserde::__xml::writer::XmlEvent::Characters(&self.str)).map_err(|e| {
                    e.to_string()
                })
            }

            fn serialize_attributes(
                &self,
                _attributes: Vec<xml::attribute::OwnedAttribute>,
                _namespace: xml::namespace::Namespace,
            ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
                Ok((Vec::new(), xml::namespace::Namespace::empty()))
            }
        }
    };

    TokenStream::from(expanded)
}


