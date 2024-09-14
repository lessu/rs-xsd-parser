use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    attribute::Attribute,
    any_attribute::AnyAttribute,
};

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

    #[yaserde(attribute)]
    pub ref_v: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>, // 处理多个 attribute

    #[yaserde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>, // 处理多个 attributeGroup

    #[yaserde(rename = "anyAttribute")]
    pub any_attribute: Option<AnyAttribute>, // 可选的 anyAttribute
}