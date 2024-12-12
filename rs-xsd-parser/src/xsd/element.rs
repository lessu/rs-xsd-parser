use yaserde::*;
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
use super::types::Types;
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub enum TypeComponent{
    #[default]
    None,

    #[yaserde(rename = "simpleType", prefix = "xs")]
    SimpleType(SimpleType),
    
    #[yaserde(rename = "complexType", prefix = "xs")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "element",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Element {
    #[yaserde(attribute = true,default = "default_false")]
    pub abstract_v: bool,

    #[yaserde(attribute = true, rename = "block")]
    pub block: Option<String>,

    #[yaserde(attribute = true)]
    pub default: Option<String>,

    #[yaserde(attribute = true, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute = true)]
    pub fixed: Option<String>,

    #[yaserde(attribute = true)]
    pub form: Option<Form>,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs", default = "default_u32_1")]
    pub max_occurs: u32,

    #[yaserde(attribute = true, rename = "minOccurs", default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true,default = "default_false")]
    pub nillable: bool,

    #[yaserde(attribute = true, rename = "ref")]
    pub ref_v: Option<String>,

    #[yaserde(attribute = true, rename = "substitutionGroup")]
    pub substitution_group: Vec<String>,

    #[yaserde(attribute = true, rename = "targetNamespace")]
    pub target_namespace: Option<String>,

    #[yaserde(attribute = true, rename = "type")]
    pub type_v: Option<QName<Types>>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub type_component: TypeComponent,

    #[yaserde(rename = "alternative", prefix = "xs")]
    pub alternatives: Vec<Alternative>,

    #[yaserde(rename = "unique")]
    pub unique: Vec<Unique>,

    #[yaserde(rename = "key")]
    pub key: Vec<Key>,

    #[yaserde(rename = "keyref")]
    pub keyref: Vec<KeyRef>,
}

