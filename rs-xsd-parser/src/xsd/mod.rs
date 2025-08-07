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
use std::fmt::Write;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Cursor, Read, Seek};
use std::str::FromStr;
use quick_xml;

#[derive(Clone, Debug)]
pub struct Xsd {
    pub name: String,
    // pub namespace: Namespace,
    pub schema: schema::Schema,
}

pub trait Traceable {
    fn print_trace(&mut self, f: &mut String);
}

impl<R> Traceable for BufReader<R> where R: Read + Seek {
    fn print_trace(&mut self, f: &mut String) {
        // seek back to 3 lines before the current position
        // const BACK_LINES: i64 = 0;
        // let mut back_lines = BACK_LINES;
        // loop{
        //     let mut c: [u8; 1] = [0];
        //     if self.seek_relative(-1).is_err(){
        //         break;
        //     }
        //     if self.read(&mut c).is_err(){
        //         break;
        //     }
        //     if c[0] == b'\n'{
        //         back_lines -= 1;
        //     }
        //     if back_lines == 0{
        //         break;
        //     }
        //     if self.seek_relative(-1).is_err(){
        //         break;
        //     }
        // }

        // print 10 lines of context
        for i in 0..10{
            let mut line = String::new();
            if self.read_line(&mut line).is_ok(){
                f.write_str(&format!("{:>4}: {}", i, line)).unwrap();
            } else {
                break;
            }
        }
    }
}

impl Xsd {
    // pub fn read_namespace(content: &str) -> Namespace {
    //     let cursor = Cursor::new(content);
    //     let parser = EventReader::new(cursor);
    //     let mut xmlns_map: Option<Namespace> = None;

    //     for event in parser {
    //         match event {
    //             Ok(XmlEvent::StartElement { attributes: _ , name, namespace, ..}) => {
    //                 if name.local_name.ends_with("schema"){
    //                     xmlns_map = Some(namespace);
    //                     break;
    //                 }
    //             }
    //             Err(_) => {
    //                 break;
    //             }
    //             _ => {}
    //         }
    //     }
    
    //     xmlns_map.unwrap()
    // }

    pub fn new(
        name: String,
        content: &str,
    ) -> Result<Self, String> {
        // read the namespace info xmlns from root node by xml-rs
        // let f = File::open(path);
        // if let Err(e) = f {
        //     return Err(format!("Error opening file: {} {}",path, e));
        // }
        // let mut reader = BufReader::new(f.unwrap());
        let cursor = Cursor::new(content.as_bytes());
        let mut reader = BufReader::new(cursor);
        let schema: Result<schema::Schema,_> = quick_xml::de::from_reader(&mut reader);
        match schema {
            Ok(schema) => {
                return Ok(Xsd {
                    name,
                    // namespace: Xsd::read_namespace(content),
                    schema,
                });
            }
            Err(e) => {
                let mut output = String::new();
                reader.print_trace(&mut output);
                return Err(format!("{}:\n {}", e.to_string(), output));
            }
        }
    }

    pub fn new_from_file(
        name: &str,
        source: &str
    ) -> Result<Self, String> {
        // read the namespace info xmlns from root node by xml-rs
        let f = File::open(source);
        if let Err(e) = f {
            return Err(format!("Error opening file: {} {}",source, e));
        }
        let mut reader = BufReader::new(f.unwrap());
        let schema: Result<schema::Schema,_> = quick_xml::de::from_reader(&mut reader);
        match schema {
            Ok(schema) => {
                return Ok(Xsd {
                    name: name.to_string(),
                    // namespace: Xsd::read_namespace(content),
                    schema,
                });
            }
            Err(e) => {
                let mut output = String::new();
                reader.print_trace(&mut output);
                return Err(format!("{}:\n {}", e.to_string(), output));
            }
        }

        // let content = {
        //     let path = std::env::current_dir().unwrap();
        //     log::info!("The current directory is {}", path.display());

        //     fs::read_to_string(source).map_err(|e| e.to_string())?
        // };

        // // skip BOM header, can be present on some files
        // let content = if content.as_bytes()[0..3] == [0xef, 0xbb, 0xbf] {
        //     content[3..].to_owned()
        // } else {
        //     content
        // };

        // Xsd::new(String::from_str(name).unwrap(), &content)
    }

    // pub fn resolve_namespace<'a>(&'a self,name: &'a str) -> Result<(&'a str,&'a str),String> {
    //     if let Some(r) =  name.split_once(':'){
    //         if let Some(namespace) = self.namespace.get(r.0) {
    //             Ok((namespace, r.1))
    //         }else{
    //             Err("unknown namespace".to_string())
    //         }
    //     } else {
    //         return match self.schema.target_namespace.as_ref() {
    //             Some(namespace) => {
    //                 Ok((namespace, name))
    //             }
    //             _ => {
    //                 Ok(("", name))
    //             }
    //         }
    //     }
    // }
}
