use serde::Deserialize;

use crate::xsd::default_fn::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    element::Element,
    group::Group,
    max_occurences::MaxOccurences
};

/**
 * <all
 *   id = ID
 *   maxOccurs = (0 | 1) : 1
 *   minOccurs = (0 | 1) : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | any | group)*)
 * </all>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "element", default)]
    pub elements: Vec<Element>,

    #[serde(rename = "any", default)]
    pub any: Vec<Any>,

    #[serde(rename = "group", default)]
    pub groups: Vec<Group>,
}
/**
 * <choice
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | group | choice | sequence | any)*)
 * </choice>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs",default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "element", default)]
    pub elements: Vec<Element>,

    #[serde(rename = "group", default)]
    pub groups: Vec<Group>,

    #[serde(rename = "choice", default)]
    pub choices: Vec<Choice>,

    #[serde(rename = "sequence", default)]
    pub sequences: Vec<Sequence>,

    #[serde(rename = "any", default)]
    pub any: Vec<Any>,
}

/**
 * <sequence
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | group | choice | sequence | any)*)
 * </sequence>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs",default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "element", default)]
    pub elements: Vec<Element>,

    #[serde(rename = "group", default)]
    pub groups: Vec<Group>,

    #[serde(rename = "choice", default)]
    pub choices: Vec<Choice>,

    #[serde(rename = "sequence", default)]
    pub sequences: Vec<Sequence>,

    #[serde(rename = "any", default)]
    pub any: Vec<Any>,
}