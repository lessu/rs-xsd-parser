use yaserde::*;
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "all",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct All {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs", default = "default_u32_1")]
    pub max_occurs: u32,

    #[yaserde(attribute = true, rename = "minOccurs", default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element", prefix = "xs")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "any", prefix = "xs")]
    pub any: Vec<Any>,

    #[yaserde(rename = "group", prefix = "xs")]
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Choice {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs",default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element", prefix = "xs")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "group", prefix = "xs")]
    pub groups: Vec<Group>,

    #[yaserde(rename = "choice", prefix = "xs")]
    pub choices: Vec<Choice>,

    #[yaserde(rename = "sequence", prefix = "xs")]
    pub sequences: Vec<Sequence>,

    #[yaserde(rename = "any", prefix = "xs")]
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Sequence {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs",default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element", prefix = "xs")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "group", prefix = "xs")]
    pub groups: Vec<Group>,

    #[yaserde(rename = "choice", prefix = "xs")]
    pub choices: Vec<Choice>,

    #[yaserde(rename = "sequence", prefix = "xs")]
    pub sequences: Vec<Sequence>,

    #[yaserde(rename = "any", prefix = "xs")]
    pub any: Vec<Any>,
}