use yaserde::*;

use crate::xsd::{
    sequence::{All, Choice, Sequence},
    annotation::Annotation,
};

use super::common_type::QName;


/**
 * <group
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   name = NCName
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (all | choice | sequence)?)
 * </group>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "group",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Group {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs")]
    pub max_occurs: Option<u32>,

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>, 

    #[yaserde(attribute)]
    pub name: Option<String>, // NCName

    #[yaserde(attribute, rename = "ref")]
    pub ref_v: Option<QName<Group>>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub componenet: GroupComponenet
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum GroupComponenet {
    #[default]
    None,
    #[yaserde(rename = "All", prefix = "xs")]
    All(All),
    #[yaserde(rename = "Choice", prefix = "xs")]
    Choice(Choice),
    #[yaserde(rename = "Sequence", prefix = "xs")]
    Sequence(Sequence),
}
