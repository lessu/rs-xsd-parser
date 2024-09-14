use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    group::Group,
    sequence::All,
    sequence::Choice,
    sequence::Sequence,
    attribute::Attribute,
    attribute_group::AttributeGroup,
    any_attribute::AnyAttribute,
    type_def::Assert,
    type_def::Restriction,
    open_content::OpenContent, 
};

/**
 * <complexContent
 *   id = ID
 *   mixed = boolean
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (restriction | extension))
 * </complexContent>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "complexContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mixed: Option<bool>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "restriction")]
    pub restriction: Option<Restriction>,

    #[yaserde(rename = "extension")]
    pub extension: Option<Extension>,
}


/**
 * <extension
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, ((attribute | attributeGroup)*, anyAttribute?), assert*)
 * </extension>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "extension",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
    #[yaserde(attribute)]
    pub base: Option<String>, // QName

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "openContent")]
    pub open_content: Option<OpenContent>,

    #[yaserde(rename = "group")]
    pub group: Option<Group>,

    #[yaserde(rename = "all")]
    pub all: Option<All>,

    #[yaserde(rename = "choice")]
    pub choice: Option<Choice>,

    #[yaserde(rename = "sequence")]
    pub sequence: Option<Sequence>,

    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[yaserde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[yaserde(rename = "anyAttribute")]
    pub any_attributes: Vec<AnyAttribute>,

    #[yaserde(rename = "assert")]
    pub asserts: Vec<Assert>,
}
