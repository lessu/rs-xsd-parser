use yaserde::*;

use crate::xsd::annotation::Annotation;

use super::any::Any;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "openContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct OpenContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mode: Option<Mode>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "any")]
    pub any: Option<Any>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Mode {
    #[default]
    #[yaserde(rename = "none")]
    None,

    #[yaserde(rename = "interleave")]
    Interleave,

    #[yaserde(rename = "suffix")]
    Suffix,
}
