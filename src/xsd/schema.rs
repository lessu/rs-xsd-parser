use yaserde::*;
use crate::xsd::{
  attribute,
  attribute_group,
  complex_type,
  element,
  group,
  import,
  simple_type,
  type_def::Form
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Schema {
  #[yaserde(rename = "targetNamespace", attribute)]
  pub target_namespace: Option<String>,
  #[yaserde(rename = "elementFormDefault", attribute)]
  pub element_form_default: Option<Form>,
  #[yaserde(rename = "attributeFormDefault", attribute)]
  pub attribute_form_default: Option<Form>,
  #[yaserde(rename = "import", prefix="xs")]
  pub imports: Vec<import::Import>,
  #[yaserde(rename = "element", prefix="xs")]
  pub elements: Vec<element::Element>,
  #[yaserde(rename = "simpleType", prefix="xs")]
  pub simple_type: Vec<simple_type::SimpleType>,
  #[yaserde(rename = "complexType", prefix="xs")]
  pub complex_type: Vec<complex_type::ComplexType>,
  #[yaserde(rename = "attribute", prefix="xs")]
  pub attributes: Vec<attribute::Attribute>,
  #[yaserde(rename = "attributeGroup", prefix="xs")]
  pub attribute_group: Vec<attribute_group::AttributeGroup>,
  #[yaserde(rename = "group", prefix  = "xs")]
  pub group: Vec<group::Group>,
}


// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn default_schema_implementation() {
//     let schema = Schema::default();

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = format!("{}", schema.implement(&TokenStream::new(), &None, &context));
//     assert_eq!(implementation, "pub mod xml_schema_types { }");
//   }

//   #[test]
//   #[should_panic]
//   fn missing_prefix() {
//     let schema = Schema {
//       target_namespace: Some("http://example.com".to_string()),
//       ..Default::default()
//     };

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     schema.implement(&TokenStream::new(), &None, &context);
//   }

//   #[test]
//   #[should_panic]
//   fn missing_target_namespace() {
//     let schema = Schema::default();

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     schema.implement(&TokenStream::new(), &Some("ex".to_string()), &context);
//   }

//   #[test]
//   fn generate_namespace() {
//     let definition = generate_namespace_definition(
//       &Some("prefix".to_string()),
//       &Some("http://example.com".to_string()),
//     );

//     let implementation = format!("{definition}");

//     assert_eq!(
//       implementation,
//       r#"# [yaserde (prefix = "prefix" , namespace = "prefix: http://example.com")]"#
//     );
//   }
// }
