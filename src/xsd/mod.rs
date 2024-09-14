mod annotation;
mod any_attribute;
mod attribute;
mod attribute_group;
mod complex_content;
mod complex_type;
mod element;
mod group;
mod import;
mod max_occurences;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod type_def;
mod any;
mod open_content;
mod default_fn;
use std::fs;
use std::str::FromStr;
use yaserde::de::from_str;


#[derive(Clone, Debug)]
pub struct Xsd {
  pub name: String,
  pub schema: schema::Schema,
}

impl Xsd {
  pub fn new(
    name: String,
    content: &str,
  ) -> Result<Self, String> {
    let schema: schema::Schema = from_str(content)?;

    Ok(Xsd {
      name,
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
