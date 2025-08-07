use yaserde::*;
use crate::xsd::default_fn::*;
use crate::xsd::{
    sequence::{All, Choice, Sequence},
    annotation::Annotation,
};

use super::{common_type::QName, max_occurences::MaxOccurences};


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
#[derive(Clone, Default, Debug, YaDeserialize)]
#[yaserde(
    rename = "group",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Group {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences, 

    #[yaserde(attribute = true)]
    pub name: Option<String>, // NCName

    #[yaserde(attribute = true, rename = "ref")]
    pub ref_v: Option<QName<Group>>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub componenet: GroupComponenet
}

#[derive(Clone, Default, Debug, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub enum GroupComponenet {
    #[default]
    None,
    #[yaserde(rename = "all", prefix = "xs")]
    All(All),
    #[yaserde(rename = "choice", prefix = "xs")]
    Choice(Choice),
    #[yaserde(rename = "sequence", prefix = "xs")]
    Sequence(Sequence),
}
