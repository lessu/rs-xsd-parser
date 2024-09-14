use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    element::Element,
    group::Group,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "all",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct All {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs")]
    pub max_occurs: Option<u32>,

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>,

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

    #[yaserde(attribute, rename = "maxOccurs")]
    pub max_occurs: Option<String>,

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>,

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

    #[yaserde(attribute, rename = "maxOccurs")]
    pub max_occurs: Option<String>,

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>,

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