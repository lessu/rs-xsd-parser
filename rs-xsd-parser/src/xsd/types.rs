use serde::Deserialize;
use crate::xsd::default_fn::*;
use crate::xsd::{
    content::{SimpleContent, ComplexContent, OpenContent},
    annotation::Annotation,
    type_def::{Assert, Assertion, ComplexChildren, Enumeration, ExplicitTimezone, FractionDigits, Length, MaxExclusive, MaxInclusive, MaxLength, MinExclusive, MinInclusive, MinLength, Pattern, TotalDigits, WhiteSpace}
};

use super::atomic_type::BaseType;
use super::attribute::{AnyAttribute, Attribute, RefAttributeGroup};
use super::common_type::QName;

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComplexTypeContent{
    #[default]
    None,

    #[serde(rename = "simpleContent")]
    SimpleContent(SimpleContent),

    #[serde(rename = "complexContent")]
    ComplexContent(ComplexContent),

    #[serde(rename = "openContent")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    #[serde(rename = "@abstract", default = "default_false")]
    pub abstract_v: bool,

    #[serde(rename = "@block")]
    pub block: Option<String>,

    #[serde(rename = "@final")]
    pub final_v: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@mixed")]
    pub mixed: Option<bool>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@defaultAttributesApply", default = "default_true")]
    pub default_attributes_apply: bool,

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    #[serde(flatten)]
    pub content: ComplexTypeContent,

    #[serde(flatten)]
    pub complex_children: ComplexChildren,

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<RefAttributeGroup>,

    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,

    #[serde(rename = "assert")]
    pub assert: Vec<Assert>

}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimpleTypeComponenet {
    #[default]
    None,
    #[serde(rename = "restriction")]
    Restriction(SimpleTypeRestriction),
    #[serde(rename = "list")]
    List(List),
    #[serde(rename = "union")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleType {
    #[serde(rename = "@final")]
    pub final_v: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>, // NCName

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    #[serde(rename = "$value")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTypeRestriction {
    #[serde(rename = "@base")]
    pub base: Option<QName<BaseType>>, // QName

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    // Use Vec to avoid nesting reference
    #[serde(rename = "simpleType")]
    pub simple_type: Vec<SimpleType>,

    #[serde(rename = "minExclusive")]
    pub min_exclusive: Option<MinExclusive>,

    #[serde(rename = "minInclusive")]
    pub min_inclusive: Option<MinInclusive>,

    #[serde(rename = "maxExclusive")]
    pub max_exclusive: Option<MaxExclusive>,

    #[serde(rename = "maxInclusive")]
    pub max_inclusive: Option<MaxInclusive>,

    #[serde(rename = "totalDigits")]
    pub total_digits: Option<TotalDigits>,

    #[serde(rename = "fractionDigits")]
    pub fraction_digits: Option<FractionDigits>,

    #[serde(rename = "length")]
    pub length: Option<Length>,

    #[serde(rename = "minLength")]
    pub min_length: Option<MinLength>,

    #[serde(rename = "maxLength")]
    pub max_length: Option<MaxLength>,

    #[serde(rename = "enumeration")]
    pub enumeration: Vec<Enumeration>,

    #[serde(rename = "whiteSpace")]
    pub white_space: Vec<WhiteSpace>,

    #[serde(rename = "pattern")]
    pub pattern: Vec<Pattern>,

    #[serde(rename = "assertion")]
    pub assertion: Vec<Assertion>,

    #[serde(rename = "explicitTimezone")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@itemType")]
    pub item_type: Option<String>, // QName

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    // use Vec to avoid nesting reference
    #[serde(rename = "simpleType")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Union {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@memberTypes")]
    pub member_types: Vec<String>, // List of QName

    #[serde(rename = "annotation")]
    pub annotation: Option<Annotation>,

    #[serde(rename = "simpleType")]
    pub simple_types: Vec<SimpleType>,
}




#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Types{
    None,
    Simple(SimpleType),
    Complex(ComplexType)
}
