use yaserde::*;

use crate::xsd::{
  types::SimpleType,
  annotation::Annotation,
  type_def::Form,
};

use super::type_def::ProcessContents;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AttributeType{
  #[yaserde(rename = "attribute", prefix = "xs")]
  pub attributes: Vec<Attribute>,

  #[yaserde(rename = "attributeGroup", prefix = "xs")]
  pub attribute_groups: Vec<AttributeGroup>,

  #[yaserde(rename = "anyAttribute", prefix = "xs")]
  pub any_attributes: Option<AnyAttribute>,
}

/**
 * <attribute
 *  default = string
 *  fixed = string
 *  form = (qualified | unqualified)
 *  id = ID
 *  name = NCName
 *  ref = QName
 *  targetNamespace = anyURI
 *  type = QName
 *  use = (optional | prohibited | required) : optional
 *  inheritable = boolean
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, simpleType?)
 *</attribute>
 */
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

  #[yaserde(rename = "use", attribute, default)]
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


/**
 * <attributeGroup
 *   id = ID
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </attributeGroup>
 * <attributeGroup
 *   id = ID
 *   name = NCName
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, ((attribute | attributeGroup)*, anyAttribute?))
 * </attributeGroup>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "attributeGroup",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AttributeGroup {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>, // NCName

    #[yaserde(attribute, rename = "ref")]
    pub ref_v: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub attributes: Option<AttributeType>,
}


/**
 * <anyAttribute
 *   id = ID
 *   namespace = ((##any | ##other) | List of (anyURI | (##targetNamespace | ##local)) )
 *   notNamespace = List of (anyURI | (##targetNamespace | ##local))
 *   notQName = List of (QName | ##defined)
 *   processContents = (lax | skip | strict) : strict
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </anyAttribute>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "anyAttribute",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AnyAttribute {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub namespace: Option<String>,

    #[yaserde(attribute, rename = "notNamespace")]
    pub not_namespace: Vec<String>,

    #[yaserde(attribute, rename = "notQName")]
    pub not_qname: Vec<String>,

    #[yaserde(attribute, rename = "processContents",default)]
    pub process_contents: ProcessContents,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
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