use yaserde::*;
use crate::xsd::default_fn::*;
use crate::xsd::{
    group::Group,
    content::SimpleContent,
    content::ComplexContent,
    annotation::Annotation,
    type_def::Assert,
    sequence::{All, Choice, Sequence},
    content::OpenContent,
    type_def::Restriction
};

use super::attribute::AttributeType;



#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexTypeOpenContent{
    #[yaserde(rename = "openContent", prefix = "xs")]
    pub open_content: Option<OpenContent>,

    #[yaserde(flatten)]
    pub complex_type_enum1: Option<ComplexTypeEnum1>,

    #[yaserde(flatten)]
    pub attribute: AttributeType,

    #[yaserde(rename = "assert", prefix = "xs")]
    pub assert: Vec<Assert>
}


#[derive(Clone, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum ComplexTypeEnum1{
    #[yaserde(rename = "group", prefix = "xs")]
    Group(Group),
    
    #[yaserde(rename = "all", prefix = "xs")]
    All(All),
    
    #[yaserde(rename = "choice", prefix = "xs")]
    Choice(Choice),

    #[yaserde(rename = "sequence", prefix = "xs")]
    Sequence(Sequence)
}
impl Default for ComplexTypeEnum1 {
    fn default() -> Self {
        ComplexTypeEnum1::Group(Group::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexTypeContent{
    /**
     * 3 pick 1
     */
    #[yaserde(rename = "simpleContent", prefix = "xs")]
    pub simple_content: Option<SimpleContent>,

    #[yaserde(rename = "complexContent", prefix = "xs")]
    pub complex_content: Option<ComplexContent>,

    #[yaserde(flatten)]
    pub open_content: Option<ComplexTypeOpenContent>,
    
}
/**
 * <complexType
 *  abstract = boolean : false
 *  block = (#all | List of (extension | restriction))
 *  final = (#all | List of (extension | restriction))
 *  id = ID
 *  mixed = boolean
 *  name = NCName
 *  defaultAttributesApply = boolean : true
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, (simpleContent | complexContent | (openContent?, (group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?), assert*)))
 * </complexType>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "complexType",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexType {
    #[yaserde(attribute, rename = "abstract", default = "default_false")]
    pub abstract_v: bool,

    #[yaserde(attribute, rename = "block")]
    pub block: Option<String>,

    #[yaserde(attribute, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mixed: Option<bool>,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "defaultAttributesApply", default = "default_true")]
    pub default_attributes_apply: bool,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub content: ComplexTypeContent

}

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum SimpleTypeComponenet {
    #[yaserde(rename = "restriction", prefix = "xs")]
    Restriction(Restriction),
    #[yaserde(rename = "list", prefix = "xs")]
    List(List),
    #[yaserde(rename = "union", prefix = "xs")]
    Union(Union),
}
impl Default for SimpleTypeComponenet {
    fn default() -> Self {
        SimpleTypeComponenet::Restriction(Restriction::default())
    }
}

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

    #[yaserde(flatten)]
    pub restriction: SimpleTypeComponenet
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

    // use Vec to avoid nesting reference
    #[yaserde(rename = "simpleType")]
    pub simple_type: Vec<SimpleType>,
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
    pub simple_types: Vec<SimpleType>,
}
