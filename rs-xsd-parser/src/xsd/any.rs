use yaserde::*;
use crate::xsd::default_fn::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::ProcessContents,
    max_occurences::MaxOccurences
};

/**
 * <any
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   namespace = ((##any | ##other) | List of (anyURI | (##targetNamespace | ##local)) )
 *   notNamespace = List of (anyURI | (##targetNamespace | ##local))
 *   notQName = List of (QName | (##defined | ##definedSibling))
 *   processContents = (lax | skip | strict) : strict
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </any>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "any",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Any {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs", default = "default_u32_1" )]
    pub min_occurs: u32,

    #[yaserde(attribute = true)]
    pub namespace: Option<String>,

    #[yaserde(attribute = true, rename = "notNamespace")]
    pub not_namespace: Vec<String>,

    #[yaserde(attribute = true, rename = "notQName")]
    pub not_qname: Vec<String>,

    #[yaserde(attribute = true, rename = "processContents")]
    pub process_contents: ProcessContents,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
