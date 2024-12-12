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
  rename="schema",
  prefix="xs",
  namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" },
)]
pub struct Schema {
  #[yaserde(attribute = true)]
  pub xmlns : Option<String>,
  
  #[yaserde(rename = "targetNamespace", attribute = true)]
  pub target_namespace: Option<String>,

  #[yaserde(rename = "elementFormDefault", attribute = true)]
  pub element_form_default: Option<Form>,

  #[yaserde(rename = "attributeFormDefault", attribute = true)]
  pub attribute_form_default: Option<Form>,

  #[yaserde(rename = "import", prefix="xs")]
  pub imports: Vec<import::Import>,

  #[yaserde(rename = "element", prefix="xs")]
  pub elements: Vec<element::Element>,

  #[yaserde(rename = "simpleType", prefix="xs")]
  pub simple_type: Vec<types::SimpleType>,

  #[yaserde(rename = "complexType", prefix="xs")]
  pub complex_type: Vec<types::ComplexType>,

  #[yaserde(rename = "attribute = true", prefix="xs")]
  pub attribute: Vec<attribute::Attribute>,

  #[yaserde(rename = "attribute = trueGroup", prefix="xs")]
  pub attribute_group: Vec<attribute::AttributeGroup>,

  #[yaserde(rename = "group", prefix  = "xs")]
  pub group: Vec<group::Group>,
}
