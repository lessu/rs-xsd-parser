use yaserde::*;
use crate::xsd::{
  annotation::Annotation, complex_type::ComplexType, max_occurences::MaxOccurences,
  rust_types_mapping::RustTypesMapping, simple_type::SimpleType, XsdContext,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Element {
  #[yaserde(attribute)]
  pub name: String,
  #[yaserde(rename = "type", attribute)]
  pub kind: Option<String>,
  #[yaserde(rename = "ref", attribute)]
  pub refers: Option<String>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Option<ComplexType>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::str::FromStr;

//   static DERIVES: &str =
//     "#[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]";

//   static DOCS: &str = r#"#[doc = "Loudness measured in Decibels"]"#;

//   #[test]
//   fn extern_type() {
//     let element = Element {
//       name: "volume".to_string(),
//       kind: Some("books:volume-type".to_string()),
//       refers: None,
//       min_occurences: None,
//       max_occurences: None,
//       complex_type: None,
//       simple_type: None,
//       annotation: Some(Annotation {
//         id: None,
//         attributes: vec![],
//         documentation: vec!["Loudness measured in Decibels".to_string()],
//       }),
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = element.implement(&quote!(), &None, &context);

//     let expected = TokenStream::from_str(&format!(
//       r#"
//         {DOCS}
//         {DERIVES}
//         pub struct Volume {{
//           #[yaserde(flatten)]
//           pub content: xml_schema_types::VolumeType,
//         }}"#
//     ))
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   fn xs_string_element() {
//     let element = Element {
//       name: "volume".to_string(),
//       kind: Some("xs:string".to_string()),
//       refers: None,
//       min_occurences: None,
//       max_occurences: None,
//       complex_type: None,
//       simple_type: None,
//       annotation: Some(Annotation {
//         id: None,
//         attributes: vec![],
//         documentation: vec!["Loudness measured in Decibels".to_string()],
//       }),
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = element.implement(&quote!(), &None, &context);

//     let expected = TokenStream::from_str(&format!(
//       r#"
//         {DOCS}
//         {DERIVES}
//         pub struct Volume {{
//           #[yaserde(text)]
//           pub content: xml_schema_types::String,
//         }}"#
//     ))
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }

//   #[test]
//   fn refers_element_field_implementation() {
//     // <xs:element ref="OwnedType" />
//     let element = Element {
//       name: "".to_string(),
//       kind: None,
//       refers: Some("OwnedType".to_string()),
//       min_occurences: None,
//       max_occurences: None,
//       complex_type: None,
//       simple_type: None,
//       annotation: None,
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = element.get_field_implementation(&context, &None);

//     let expected = TokenStream::from_str(
//       r#"#[yaserde(rename = "OwnedType")] pub owned_type : xml_schema_types :: OwnedType ,"#,
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());

//     // <xs:element ref="OwnedType"  minOccurs="0" maxOccurs="unbounded" />
//     let element = Element {
//       name: "".to_string(),
//       kind: None,
//       refers: Some("OwnedType".to_string()),
//       min_occurences: Some(0),
//       max_occurences: Some(MaxOccurences::Unbounded),
//       complex_type: None,
//       simple_type: None,
//       annotation: None,
//     };

//     let implementation = element.get_field_implementation(&context, &None);

//     let expected = TokenStream::from_str(
//       r#"#[yaserde(rename = "OwnedType")] pub owned_type_list : Vec < xml_schema_types :: OwnedType > ,"#
//     )
//     .unwrap();

//     assert_eq!(implementation.to_string(), expected.to_string());
//   }
// }
