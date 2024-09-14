use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    sequence::All,
    sequence::Choice,
    sequence::Sequence,
};

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
    pub max_occurs: Option<u32>, // 处理 nonNegativeInteger 或 unbounded

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>, // 处理 nonNegativeInteger

    #[yaserde(attribute)]
    pub name: Option<String>, // NCName

    #[yaserde(attribute)]
    pub ref_v: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "all")]
    pub all: Option<All>,

    #[yaserde(rename = "choice")]
    pub choice: Option<Choice>,

    #[yaserde(rename = "sequence")]
    pub sequence: Option<Sequence>,
}