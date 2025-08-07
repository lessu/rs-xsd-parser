use serde::Deserialize;

use crate::xsd::{
  attribute,
  types,
  element,
  group,
  import,
  type_def::Form
};

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
  #[serde(rename = "@xmlns")]
  pub xmlns : Option<String>,
  
  #[serde(rename = "@targetNamespace")]
  pub target_namespace: Option<String>,

  #[serde(rename = "@elementFormDefault")]
  pub element_form_default: Option<Form>,

  #[serde(rename = "@attributeFormDefault")]
  pub attribute_form_default: Option<Form>,

  #[serde(rename = "import", default)]
  pub imports: Vec<import::Import>,

  #[serde(rename = "element", default)]
  pub elements: Vec<element::Element>,

  #[serde(rename = "simpleType", default)]
  pub simple_type: Vec<types::SimpleType>,

  #[serde(rename = "complexType", default)]
  pub complex_type: Vec<types::ComplexType>,

  #[serde(rename = "attribute", default)]
  pub attribute: Vec<attribute::Attribute>,

  #[serde(rename = "attributeGroup", default)]
  pub attribute_group: Vec<attribute::AttributeGroup>,

  #[serde(rename = "group", default)]
  pub group: Vec<group::Group>,
}
