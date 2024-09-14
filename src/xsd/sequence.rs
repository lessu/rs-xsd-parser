use yaserde::*;
use crate::xsd::default_fn::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    element::Element,
    group::Group,
};

use super::max_occurences::MaxOccurences;

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