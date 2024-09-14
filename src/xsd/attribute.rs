use yaserde::*;

use crate::xsd::{
  rust_types_mapping::RustTypesMapping, simple_type::SimpleType, XsdContext,
};
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attribute",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Attribute {
  #[yaserde(prefix = "xs", attribute)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute)]
  pub kind: Option<String>,
  // #[yaserde(attribute)]
  // pub default: Option<String>,
  // #[yaserde(attribute)]
  // pub fixed: Option<String>,
  #[yaserde(rename = "use", attribute)]
  pub required: Required,
  #[yaserde(rename = "ref", attribute)]
  pub reference: Option<String>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Required {
  #[default]
  #[yaserde(rename = "optional")]
  Optional,
  #[yaserde(rename = "required")]
  Required,
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::str::FromStr;

//   #[test]
//   fn default_required() {
//     assert_eq!(Required::default(), Required::Optional);
//   }

//   #[test]
//   fn string_attribute() {
//     let attribute = Attribute {
//       name: Some("language".to_string()),
//       kind: Some("xs:string".to_string()),
//       reference: None,
//       required: Required::Required,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = attribute.implement(&TokenStream::new(), &None, &context);

//     let expected = TokenStream::from_str(
//       r#"
//         #[yaserde(attribute)]
//         pub language: String,
//       "#,
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   fn optional_string_attribute() {
//     let attribute = Attribute {
//       name: Some("language".to_string()),
//       kind: Some("xs:string".to_string()),
//       reference: None,
//       required: Required::Optional,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = attribute.implement(&TokenStream::new(), &None, &context);

//     let expected = TokenStream::from_str(
//       r#"
//         #[yaserde(attribute)]
//         pub language: Option<String> ,
//       "#,
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   fn type_attribute() {
//     let attribute = Attribute {
//       name: Some("type".to_string()),
//       kind: Some("xs:string".to_string()),
//       reference: None,
//       required: Required::Optional,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = attribute.implement(&TokenStream::new(), &None, &context);

//     let expected = TokenStream::from_str(
//       r#"
//         #[yaserde(attribute, rename="type")]
//         pub kind: Option<String> ,
//       "#,
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   fn reference_type_attribute() {
//     let attribute = Attribute {
//       name: Some("type".to_string()),
//       kind: None,
//       reference: Some("MyType".to_string()),
//       required: Required::Optional,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = attribute.implement(&TokenStream::new(), &None, &context);

//     let expected = TokenStream::from_str(
//       r#"
//         #[yaserde(attribute, rename="type")]
//         pub kind: Option<MyType> ,
//       "#,
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   #[should_panic]
//   fn bad_type_attribute() {
//     let attribute = Attribute {
//       name: Some("type".to_string()),
//       kind: None,
//       reference: None,
//       required: Required::Optional,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     attribute.implement(&TokenStream::new(), &None, &context);
//   }

//   #[test]
//   fn attribute_without_name() {
//     let attribute = Attribute {
//       name: None,
//       kind: Some("xs:string".to_string()),
//       reference: None,
//       required: Required::Optional,
//       simple_type: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = attribute
//       .implement(&TokenStream::new(), &None, &context)
//       .to_string();
//     assert!(implementation.is_empty());
//   }
// }
