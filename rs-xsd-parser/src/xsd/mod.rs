pub mod annotation;
pub mod attribute;
pub mod content;
pub mod types;
pub mod element;
pub mod group;
pub mod import;
pub mod max_occurences;
pub mod schema;
pub mod sequence;
pub mod type_def;
pub mod any;
pub mod default_fn;
pub mod atomic_type;
pub mod common_type;
use std::fs;
use std::io::Cursor;
use std::str::FromStr;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;
use xml::EventReader;
use yaserde::de::from_str;


#[derive(Clone, Debug)]
pub struct Xsd {
    pub name: String,
    pub namespace: Namespace,
    pub schema: schema::Schema,
}

impl Xsd {
    pub fn read_namespace(content: &str) -> Namespace {
        let cursor = Cursor::new(content);
        let parser = EventReader::new(cursor);
        let mut xmlns_map: Option<Namespace> = None;

        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { attributes: _ , name, namespace, ..}) => {
                    if name.local_name.ends_with("schema"){
                        xmlns_map = Some(namespace);
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
        }
    
        xmlns_map.unwrap()
    }

    pub fn new(
        name: String,
        content: &str,
    ) -> Result<Self, String> {
        // read the namespace info xmlns from root node by xml-rs
        let schema: schema::Schema = from_str(content)?;

        Ok(Xsd {
            name,
            namespace: Xsd::read_namespace(content),
            schema,
        })
    }

    pub fn new_from_file(
        name: &str,
        source: &str
    ) -> Result<Self, String> {
        let content = {
            let path = std::env::current_dir().unwrap();
            log::info!("The current directory is {}", path.display());

            fs::read_to_string(source).map_err(|e| e.to_string())?
        };

        // skip BOM header, can be present on some files
        let content = if content.as_bytes()[0..3] == [0xef, 0xbb, 0xbf] {
            content[3..].to_owned()
        } else {
            content
        };

        Xsd::new(String::from_str(name).unwrap(), &content)
    }

}
