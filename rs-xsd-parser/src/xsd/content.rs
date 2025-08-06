use serde::Deserialize;

use crate::xsd::{
    annotation::Annotation, any::Any, type_def::{Assert, Assertion, ComplexChildren, Enumeration, FractionDigits, Length, MaxExclusive, MaxInclusive, MaxLength, MinExclusive, MinInclusive, MinLength, Pattern, TotalDigits, WhiteSpace}, types::SimpleType
};

use super::{atomic_type::BaseType, attribute::{AnyAttribute, Attribute, AttributeGroup}, common_type::QName};


/**
 * <complexContent
 *   id = ID
 *   mixed = boolean
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (restriction | extension))
 * </complexContent>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexContent {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@mixed")]
    pub mixed: Option<bool>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "$value")]
    pub value: ComplexContextRestrictionOrExtrension,

}


#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComplexContextRestrictionOrExtrension {
    #[default]
    Undefined,
    Restriction(ComplexContentRestriction),
    Extension(ComplexContentExtension),
}
/**
 * <restriction
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, openContent?, (group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?), assert*)
 * </restriction>
 */

 #[derive(Clone, Default, Debug, Deserialize)]
 #[serde(rename_all = "camelCase")]
 pub struct ComplexContentRestriction {
    #[serde(rename = "@base")]
    pub base: Option<QName<BaseType>>, // QName

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "openContet")]
    pub open_context: Option<OpenContent>,
 
    #[serde(flatten)]
    pub complex_children: ComplexChildren,

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,

    #[serde(rename = "assert")]
    pub  assert: Vec<Assert>,
 }
 
/**
 * <extension
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, openContent?, ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?), assert*))
 * </extension>
 */

 #[derive(Clone, Default, Debug, Deserialize)]
 #[serde(rename_all = "camelCase")]
 pub struct ComplexContentExtension{
    #[serde(rename = "@base")]
    pub base: Option<QName<BaseType>>, // QName
 
    #[serde(rename = "@id")]
    pub id: Option<String>,
 
    #[serde()]
    pub annotation: Option<Annotation>,
  
    #[serde(rename = "openContet")]
    pub open_context: Option<OpenContent>,

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,

    #[serde(flatten)]
    pub complex_children: ComplexChildren,

    #[serde(rename = "assert")]
    pub assert: Vec<Assert>,
 }

/**
 * <simpleContent
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (restriction | extension))
 * </simpleContent> 
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleContent {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "$value")]
    pub value: SimpleContextRestrictionOrExtrension,
}


#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimpleContextRestrictionOrExtrension {
    #[default]
    Undefined,
    Restriction(SimpleContentRestriction),
    Extension(SimpleContentExtension),
}

/** SimpleContent
 * <restriction
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, (simpleType?, (minExclusive | minInclusive | maxExclusive | maxInclusive | totalDigits | fractionDigits | length | minLength | maxLength | enumeration | whiteSpace | pattern | assertion | {any with namespace: ##other})*)?, ((attribute | attributeGroup)*, anyAttribute?), assert*)
 * </restriction>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleContentRestriction {
    #[serde(rename = "@base")]
    pub base: Option<QName<BaseType>>, // QName

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "simpleType")]
    pub simple_type: Option<SimpleType>,

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

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,

    #[serde(rename = "assert")]
    pub assert: Vec<Assert>,
}

/** SimpleCopntent
 * <extension
 *   base = QName
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *   Content: (annotation?, ((attribute | attributeGroup)*, anyAttribute?), assert*)
 * </extension>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleContentExtension{
    #[serde(rename = "@base")]
    pub base: Option<QName<BaseType>>, // QName

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "attributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[serde(rename = "anyAttribute")]
    pub any_attributes: Option<AnyAttribute>,

    #[serde(rename = "assert")]
    pub assert: Vec<Assert>,
}

/**
 * <openContent
 *   id = ID
 *   mode = (none | interleave | suffix) : interleave
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, any?)
 * </openContent>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenContent {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@mode")]
    pub mode: Option<Mode>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "any")]
    pub any: Option<Any>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    #[default]
    None,
    Interleave,
    Suffix,
}
