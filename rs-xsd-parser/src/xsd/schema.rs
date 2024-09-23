use yaserde::*;
use crate::xsd::{
  attribute,
  types,
  element,
  group,
  import,
  type_def::Form
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Schema {
  #[yaserde(attribute)]
  pub xmlns : Option<String>,
  
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
  pub simple_type: Vec<types::SimpleType>,

  #[yaserde(rename = "complexType", prefix="xs")]
  pub complex_type: Vec<types::ComplexType>,

  #[yaserde(rename = "attribute", prefix="xs")]
  pub attribute: Vec<attribute::Attribute>,

  #[yaserde(rename = "attributeGroup", prefix="xs")]
  pub attribute_group: Vec<attribute::AttributeGroup>,

  #[yaserde(rename = "group", prefix  = "xs")]
  pub group: Vec<group::Group>,
}
