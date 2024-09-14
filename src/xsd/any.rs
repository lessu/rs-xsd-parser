use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::ProcessContents
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "any",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Any {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs")]
    pub max_occurs: Option<u32>,

    #[yaserde(attribute, rename = "minOccurs")]
    pub min_occurs: Option<u32>,

    #[yaserde(attribute)]
    pub namespace: Option<String>,

    #[yaserde(attribute, rename = "notNamespace")]
    pub not_namespace: Vec<String>,

    #[yaserde(attribute, rename = "notQName")]
    pub not_qname: Vec<String>,

    #[yaserde(attribute, rename = "processContents")]
    pub process_contents: Option<ProcessContents>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
