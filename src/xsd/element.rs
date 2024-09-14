use yaserde::*;
use crate::xsd::default_fn::*;

use crate::xsd::{
    type_def::Alternative,
    type_def::Unique,
    type_def::Key,
    type_def::KeyRef,
    type_def::Form,
    annotation::Annotation,
    complex_type::ComplexType,
    simple_type::SimpleType
};
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "element",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Element {
    #[yaserde(attribute,default = "default_false")]
    pub abstract_: bool,

    #[yaserde(attribute, rename = "block")]
    pub block: Option<String>,

    #[yaserde(attribute)]
    pub default: Option<String>,

    #[yaserde(attribute, rename = "final")]
    pub final_: Option<String>,

    #[yaserde(attribute)]
    pub fixed: Option<String>,

    #[yaserde(attribute)]
    pub form: Option<Form>,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "maxOccurs", default = "default_u32_1")]
    pub max_occurs: u32,

    #[yaserde(attribute, rename = "minOccurs", default = "default_u32_1")]
    pub min_occurs: u32,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute,default = "default_false")]
    pub nillable: bool,

    #[yaserde(attribute, rename = "ref")]
    pub ref_: Option<String>,

    #[yaserde(attribute, rename = "substitutionGroup")]
    pub substitution_group: Vec<String>,

    #[yaserde(attribute, rename = "targetNamespace")]
    pub target_namespace: Option<String>,

    #[yaserde(attribute, rename = "type")]
    pub type_: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
    
    #[yaserde(rename = "simpleType", prefix = "xs")]
    pub simple_type:Option<SimpleType>,
    
    #[yaserde(rename = "complexType", prefix = "xs")]
    pub complex_type:Option<ComplexType>,

    #[yaserde(rename = "alternative", prefix = "xs")]
    pub alternatives: Vec<Alternative>,

    #[yaserde(rename = "unique")]
    pub unique: Vec<Unique>,

    #[yaserde(rename = "key")]
    pub key: Vec<Key>,

    #[yaserde(rename = "keyref")]
    pub keyref: Vec<KeyRef>,
}

