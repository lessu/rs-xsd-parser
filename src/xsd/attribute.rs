use yaserde::*;

use crate::xsd::{
  simple_type::SimpleType,
  annotation::Annotation,
  type_def::Form
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attribute",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Attribute {
  #[yaserde(attribute)]
  pub name: Option<String>,

  #[yaserde(attribute)]
  pub default: Option<String>,

  #[yaserde(attribute)]
  pub fixed: Option<String>,

  #[yaserde(attribute)]
  pub form: Option<Form>,

  #[yaserde(attribute)]
  pub id: Option<String>,

  #[yaserde(attribute)]
  target_namespace: Option<String>,

  #[yaserde(attribute, rename = "type")]
  pub type_v: Option<String>,

  #[yaserde(rename = "use", attribute,default)]
  pub required: Use,

  #[yaserde(rename = "ref", attribute)]
  pub ref_v: Option<String>,

  #[yaserde(rename = "simpleType", prefix = "xs")]
  pub simple_type: Option<SimpleType>,

  #[yaserde(rename = "annotation", prefix = "xs")]
  pub annotation: Option<Annotation>,
  
  #[yaserde(attribute)]
  pub inheritable: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Use {
    #[default]
    #[yaserde(rename = "optional")]
    Optional,
    #[yaserde(rename = "prohibited")]
    Prohibited,
    #[yaserde(rename = "required")]
    Required,
}