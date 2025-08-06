use serde::Deserialize;

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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Any {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,

    #[serde(rename = "@notNamespace")]
    pub not_namespace: Vec<String>,

    #[serde(rename = "@notQName")]
    pub not_qname: Vec<String>,

    #[serde(rename = "@processContents")]
    pub process_contents: ProcessContents,

    #[serde()]
    pub annotation: Option<Annotation>,
}
