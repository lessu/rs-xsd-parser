use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    attribute::Attribute,
    any_attribute::AnyAttribute,
};

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

    #[yaserde(attribute)]
    pub ref_v: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[yaserde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[yaserde(rename = "anyAttribute")]
    pub any_attribute: Option<AnyAttribute>,
}