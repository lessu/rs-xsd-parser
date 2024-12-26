use yaserde::*;
use crate::xsd::default_fn::*;
use crate::xsd::{
    content::{SimpleContent, ComplexContent, OpenContent},
    annotation::Annotation,
    attribute::RefAttributeType,
    type_def::{Assert, Assertion, ComplexChildren, Enumeration, ExplicitTimezone, FractionDigits, Length, MaxExclusive, MaxInclusive, MaxLength, MinExclusive, MinInclusive, MinLength, Pattern, TotalDigits, WhiteSpace}
};

use super::atomic_type::{BaseType};
use super::common_type::QName;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub enum ComplexTypeContent{
    #[default]
    None,

    #[yaserde(rename = "simpleContent", prefix = "xs")]
    SimpleContent(SimpleContent),

    #[yaserde(rename = "complexContent", prefix = "xs")]
    ComplexContent(ComplexContent),

    #[yaserde(rename = "openContent", prefix = "xs")]
    OpenContent(OpenContent),
    
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
 * 
 * Content means simpleContent | complexContent | openContent are optional, and only one will apear
 * if simpleContent or complexContent, then you can't add other content (like gourp, attribute, etc)
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "complexType",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct ComplexType {
    #[yaserde(attribute = true, rename = "abstract", default = "default_false")]
    pub abstract_v: bool,

    #[yaserde(attribute = true, rename = "block")]
    pub block: Option<String>,

    #[yaserde(attribute = true, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub mixed: Option<bool>,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true, rename = "defaultAttributesApply", default = "default_true")]
    pub default_attributes_apply: bool,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub content: ComplexTypeContent,

    #[yaserde(flatten = true)]
    pub complex_children: ComplexChildren,

    #[yaserde(flatten = true)]
    pub attribute: RefAttributeType,

    #[yaserde(rename = "assert", prefix = "xs")]
    pub assert: Vec<Assert>

}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub enum SimpleTypeComponenet {
    #[default]
    None,
    #[yaserde(rename = "restriction", prefix = "xs")]
    Restriction(SimpleTypeRestriction),
    #[yaserde(rename = "list", prefix = "xs")]
    List(List),
    #[yaserde(rename = "union", prefix = "xs")]
    Union(Union),
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct SimpleType {
    #[yaserde(attribute = true, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>, // NCName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub restriction: SimpleTypeComponenet
}

/** SimpleType
 * <restriction
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, (simpleType?, (minExclusive | minInclusive | maxExclusive | maxInclusive | totalDigits | fractionDigits | length | minLength | maxLength | enumeration | whiteSpace | pattern | assertion | explicitTimezone | {any with namespace: ##other})*))
 * </restriction>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "restriction",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct SimpleTypeRestriction {
    #[yaserde(attribute = true)]
    pub base: Option<QName<BaseType>>, // QName

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    // Use Vec to avoid nesting reference
    #[yaserde(rename = "simpleType", prefix = "xs")]
    pub simple_type: Vec<SimpleType>,

    #[yaserde(rename = "minExclusive", prefix = "xs")]
    pub min_exclusive: Option<MinExclusive>,

    #[yaserde(rename = "minInclusive", prefix = "xs")]
    pub min_inclusive: Option<MinInclusive>,

    #[yaserde(rename = "maxExclusive", prefix = "xs")]
    pub max_exclusive: Option<MaxExclusive>,

    #[yaserde(rename = "maxInclusive", prefix = "xs")]
    pub max_inclusive: Option<MaxInclusive>,

    #[yaserde(rename = "totalDigits", prefix = "xs")]
    pub total_digits: Option<TotalDigits>,

    #[yaserde(rename = "fractionDigits", prefix = "xs")]
    pub fraction_digits: Option<FractionDigits>,

    #[yaserde(rename = "length", prefix = "xs")]
    pub length: Option<Length>,

    #[yaserde(rename = "minLength", prefix = "xs")]
    pub min_length: Option<MinLength>,

    #[yaserde(rename = "maxLength", prefix = "xs")]
    pub max_length: Option<MaxLength>,

    #[yaserde(rename = "enumeration", prefix = "xs")]
    pub enumeration: Vec<Enumeration>,

    #[yaserde(rename = "whiteSpace", prefix = "xs")]
    pub white_space: Vec<WhiteSpace>,

    #[yaserde(rename = "pattern", prefix = "xs")]
    pub pattern: Vec<Pattern>,

    #[yaserde(rename = "assertion", prefix = "xs")]
    pub assertion: Vec<Assertion>,

    #[yaserde(rename = "explicitTimezone", prefix = "xs")]
    pub explicit_timezone: Vec<ExplicitTimezone>,

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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct List {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "itemType")]
    pub item_type: Option<String>, // QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    // use Vec to avoid nesting reference
    #[yaserde(rename = "simpleType", prefix = "xs")]
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
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Union {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "memberTypes")]
    pub member_types: Vec<String>, // List of QName

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType")]
    pub simple_types: Vec<SimpleType>,
}




#[derive(Clone, Debug, PartialEq)]
pub enum Types{
    None,
    Simple(SimpleType),
    Complex(ComplexType)
}
