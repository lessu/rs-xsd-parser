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
