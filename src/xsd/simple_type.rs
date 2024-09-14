use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::Restriction
};

/**
 * <simpleType
 *  final = (#all | List of (list | union | restriction | extension))
 *  id = ID
 *  name = NCName
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, (restriction | list | union))
 * </simpleType>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "simpleType",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleType {
    #[yaserde(attribute, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>, // NCName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    /* use vec to avoid infinite recursive reference */
    #[yaserde(rename = "restriction")]
    pub restriction: Vec<Restriction>,

    #[yaserde(rename = "list")]
    pub list: Vec<List>,

    #[yaserde(rename = "union")]
    pub union: Vec<Union>,
}

/**
 * <list
 *  id = ID
 *  itemType = QName
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, simpleType?)
 *</list>
 */
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
    pub simple_type: Option<SimpleType>,
}
/**
 * <union
 *  id = ID
 *  memberTypes = List of QName
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, simpleType*)
 *</union>
 */
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
    pub simple_types: Option<SimpleType>,
}
