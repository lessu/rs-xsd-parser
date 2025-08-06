use serde::Deserialize;

use crate::xsd::{
    types::SimpleType,
    annotation::Annotation,
    type_def::{Form, ProcessContents},
};

use super::{common_type::QName, types::Types};

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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "camelCase" )]
pub struct Attribute {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@default")]
    pub default: Option<String>,

    #[serde(rename = "@fixed")]
    pub fixed: Option<String>,

    #[serde(rename = "@form")]
    pub form: Option<Form>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@targetNamespace")]
    target_namespace: Option<String>,

    #[serde(rename = "@type")]
    pub type_v: Option<QName<Types>>,

    #[serde(rename = "@use", default="default_use")]
    pub use_v: Use,

    #[serde(rename = "@ref")]
    pub ref_v: Option<QName<Attribute>>,

    #[serde(rename = "@inheritable")]
    pub inheritable: Option<bool>,

    #[serde(rename = "simpleType")]
    pub simple_type: Option<SimpleType>,

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "camelCase" )]
pub struct AttributeGroup {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>, // NCName

    #[serde(rename = "@ref")]
    pub ref_v: Option<QName<AttributeGroup>>, // QName

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<RefAttributeGroup>,
    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,
}
/**
 * <attributeGroup
 *   id = ID
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?)
 * </attributeGroup>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "camelCase" )]

pub struct RefAttributeGroup {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@ref")]
    pub ref_v: Option<QName<AttributeGroup>>, // QName

    #[serde()]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "camelCase" )]

pub struct AnyAttribute {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,

    #[serde(rename = "@notNamespace")]
    pub not_namespace: Vec<String>,

    #[serde(rename = "@notQName")]
    pub not_qname: Vec<String>,

    #[serde(rename = "@processContents")]
    pub process_contents: ProcessContents,

    #[serde()]
    pub annotation: Option<Annotation>,

}


#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Use {
    #[default]
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "prohibited")]
    Prohibited,
    #[serde(rename = "required")]
    Required,
}
