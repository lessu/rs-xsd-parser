use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::ProcessContents
};

/**
 * <anyAttribute
 *   id = ID
 *   namespace = ((##any | ##other) | List of (anyURI | (##targetNamespace | ##local)) )
 *   notNamespace = List of (anyURI | (##targetNamespace | ##local))
 *   notQName = List of (QName | ##defined)
 *   processContents = (lax | skip | strict) : strict
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </anyAttribute>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "anyAttribute",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AnyAttribute {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub namespace: Option<String>,

    #[yaserde(attribute, rename = "notNamespace")]
    pub not_namespace: Vec<String>,

    #[yaserde(attribute, rename = "notQName")]
    pub not_qname: Vec<String>,

    #[yaserde(attribute, rename = "processContents",default)]
    pub process_contents: ProcessContents,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
