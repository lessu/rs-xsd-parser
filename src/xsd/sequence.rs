use yaserde::*;
use crate::xsd::default_fn::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    element::Element,
    group::Group,
};

use super::max_occurences::MaxOccurences;
/**
 * <all
 *   id = ID
 *   maxOccurs = (0 | 1) : 1
 *   minOccurs = (0 | 1) : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | any | group)*)
 * </all>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "all",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct All {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs", default = "default_u32_1")]
    pub max_occurs: u32,

    #[yaserde(attribute, rename = "minOccurs", default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "any")]
    pub any: Vec<Any>,

    #[yaserde(rename = "group")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "choice",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Choice {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs",default)]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute, rename = "minOccurs",default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "group")]
    pub groups: Vec<Group>,

    #[yaserde(rename = "choice")]
    pub choices: Vec<Choice>,

    #[yaserde(rename = "sequence")]
    pub sequences: Vec<Sequence>,

    #[yaserde(rename = "any")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "sequence",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Sequence {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute, rename = "minOccurs",default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "group")]
    pub groups: Vec<Group>,

    #[yaserde(rename = "choice")]
    pub choices: Vec<Choice>,

    #[yaserde(rename = "sequence")]
    pub sequences: Vec<Sequence>,

    #[yaserde(rename = "any")]
    pub any: Vec<Any>,
}