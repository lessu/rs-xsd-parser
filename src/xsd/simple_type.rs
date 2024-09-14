use yaserde::*;

use crate::xsd::annotation::Annotation;

use super::type_def::Restriction;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "simpleType",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleType {
    #[yaserde(attribute, rename = "final")]
    pub final_v: Option<String>, // 处理 #all | List of (list | union | restriction | extension)

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>, // NCName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "restriction")]
    pub restriction: Vec<Restriction>,

    #[yaserde(rename = "list")]
    pub list: Vec<List>,

    #[yaserde(rename = "union")]
    pub union: Vec<Union>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "list",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct List {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "itemType")]
    pub item_type: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType")]
    pub simple_type: Vec<SimpleType>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "union",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Union {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "memberTypes")]
    pub member_types: Vec<String>, // List of QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType")]
    pub simple_types: Vec<SimpleType>, // 处理多个 simpleType
}
