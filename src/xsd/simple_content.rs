use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::Restriction,
    complex_content::Extension,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "simpleContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "restriction")]
    pub restriction: Option<Restriction>,

    #[yaserde(rename = "extension")]
    pub extension: Option<Extension>,
}
