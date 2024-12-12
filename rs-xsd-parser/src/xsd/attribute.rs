use yaserde::*;

use crate::xsd::{
    types::SimpleType,
    annotation::Annotation,
    type_def::{Form, ProcessContents},
};

use super::{common_type::QName, types::Types};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct AttributeType{
    #[yaserde(rename = "attribute = true", prefix = "xs")]
    pub attributes: Vec<Attribute>,

    #[yaserde(rename = "attribute = trueGroup", prefix = "xs")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[yaserde(rename = "anyAttribute", prefix = "xs")]
    pub any_attributes: Option<AnyAttribute>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct RefAttributeType{
    #[yaserde(rename = "attribute = true", prefix = "xs")]
    pub attributes: Vec<Attribute>,

    #[yaserde(rename = "attribute = trueGroup", prefix = "xs")]
    pub attribute_groups: Vec<RefAttributeGroup>,

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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Attribute {
    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true)]
    pub default: Option<String>,

    #[yaserde(attribute = true)]
    pub fixed: Option<String>,

    #[yaserde(attribute = true)]
    pub form: Option<Form>,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    target_namespace: Option<String>,

    #[yaserde(attribute = true, rename = "type")]
    pub type_v: Option<QName<Types>>,

    #[yaserde(rename = "use", attribute = true, default="default_use")]
    pub use_v: Use,

    #[yaserde(rename = "ref", attribute = true)]
    pub ref_v: Option<QName<Attribute>>,

    #[yaserde(rename = "simpleType", prefix = "xs")]
    pub simple_type: Option<SimpleType>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(attribute = true)]
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct AttributeGroup {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>, // NCName

    #[yaserde(attribute = true, rename = "ref")]
    pub ref_v: Option<QName<AttributeGroup>>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub attributes: RefAttributeType,
}
/**
 * <attributeGroup
 *   id = ID
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?)
 * </attributeGroup>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "attributeGroup",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct RefAttributeGroup {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "ref")]
    pub ref_v: Option<QName<AttributeGroup>>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

}
fn default_use() -> Use{
  Use::Optional
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct AnyAttribute {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub namespace: Option<String>,

    #[yaserde(attribute = true, rename = "notNamespace")]
    pub not_namespace: Vec<String>,

    #[yaserde(attribute = true, rename = "notQName")]
    pub not_qname: Vec<String>,

    #[yaserde(attribute = true, rename = "processContents")]
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
