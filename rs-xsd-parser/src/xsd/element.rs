use serde::Deserialize;
use crate::xsd::default_fn::*;

use crate::xsd::{
    type_def::Alternative,
    type_def::Unique,
    type_def::Key,
    type_def::KeyRef,
    type_def::Form,
    annotation::Annotation,
    types::ComplexType,
    types::SimpleType
};

use super::common_type::QName;
use super::max_occurences::MaxOccurences;
use super::types::Types;
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TypeComponent{
    #[default]
    None,
    SimpleType(SimpleType),
    ComplexType(ComplexType)

}

/**
 * <element
 *  abstract = boolean : false
 *  block = (#all | List of (extension | restriction | substitution))
 *  default = string
 *  final = (#all | List of (extension | restriction))
 *  fixed = string
 *  form = (qualified | unqualified)
 *  id = ID
 *  maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *  minOccurs = nonNegativeInteger : 1
 *  name = NCName
 *  nillable = boolean : false
 *  ref = QName
 *  substitutionGroup = List of QName
 *  targetNamespace = anyURI
 *  type = QName
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, ((simpleType | complexType)?, alternative*, (unique | key | keyref)*))
 * </element>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "@abstract", default = "default_false")]
    pub abstract_v: bool,

    #[serde(rename = "@block")]
    pub block: Option<String>,

    #[serde(rename = "@default")]
    pub default: Option<String>,

    #[serde(rename = "@final")]
    pub final_v: Option<String>,

    #[serde(rename = "@fixed")]
    pub fixed: Option<String>,

    #[serde(rename = "@form")]
    pub form: Option<Form>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[serde(rename = "@minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@nillable" ,default = "default_false")]
    pub nillable: bool,

    #[serde(rename = "@ref")]
    pub ref_v: Option<String>,

    #[serde(rename = "@substitutionGroup", default)]
    pub substitution_group: Vec<String>,

    #[serde(rename = "@targetNamespace")]
    pub target_namespace: Option<String>,

    #[serde(rename = "@type")]
    pub type_v: Option<QName<Types>>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(flatten)]
    pub type_component: TypeComponent,

    #[serde(rename = "alternative", default)]
    pub alternatives: Vec<Alternative>,

    #[serde(rename = "unique", default)]
    pub unique: Vec<Unique>,

    #[serde(rename = "key", default)]
    pub key: Vec<Key>,

    #[serde(rename = "keyref", default)]
    pub keyref: Vec<KeyRef>,
}

