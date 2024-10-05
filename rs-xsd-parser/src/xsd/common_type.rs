// use std::{fmt::Write};

use std::{fmt::Write, marker::PhantomData};

use yaserde::YaDeserialize;

use super::attribute::Attribute;

fn ya_xml_parse_string<R: std::io::Read>(type_name:&str ,reader: &mut yaserde::de::Deserializer<R>) -> Result<String,String>{
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
                    err = Some(format!("{} should be a text, but get a complex type", type_name).to_string());
                    break;
                }
            }
            xml::reader::XmlEvent::EndElement { name: _ } => {
                level -= 1;
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
        Ok(ret)
    }
}

/**
 * A QName is a name with an optional namespace qualification, 
 * as defined in [XML Namespaces 1.1]. 
 * When used in connection with the XML representation of schema components 
 * or references to them, this refers to the simple type QName 
 * as defined in [XML Schema: Datatypes]
 */
#[derive(Debug,Clone,PartialEq,Default)]
pub struct QName<T> {
    __own:PhantomData<T>,
    value:String
}

impl<T> YaDeserialize for QName<T>{
    fn deserialize<R: std::io::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let res = ya_xml_parse_string("QName",reader);
        if res.is_ok() {
            Ok(QName::<T>{
                __own: PhantomData::<T>,
                value: res.ok().unwrap()
            })
        }else{
            Err(res.err().unwrap())
        }
    }
}

