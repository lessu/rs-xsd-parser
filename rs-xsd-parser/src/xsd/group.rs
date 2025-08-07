use serde::Deserialize;

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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences, 

    #[serde(rename = "@name")]
    pub name: Option<String>, // NCName

    #[serde(rename = "@ref")]
    pub ref_v: Option<QName<Group>>, // QName

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    #[serde(rename = "$value")]
    pub componenet: GroupComponenet
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupComponenet {
    #[default]
    None,
    All(All),
    Choice(Choice),
    Sequence(Sequence),
}
