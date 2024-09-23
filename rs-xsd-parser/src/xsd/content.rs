use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    attribute::AttributeType,
    type_def::{Assert, Assertion, ComplexChildren, Enumeration, FractionDigits, Length, MaxExclusive, MaxInclusive, MaxLength, MinExclusive, MinInclusive, MinLength, Pattern, TotalDigits, WhiteSpace},
    types::SimpleType
};



/**
 * <complexContent
 *   id = ID
 *   mixed = boolean
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (restriction | extension))
 * </complexContent>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "complexContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mixed: Option<bool>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub value: ComplexContextRestrictionOrExtrension,

}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum ComplexContextRestrictionOrExtrension {
    #[default]
    Undefined,
    #[yaserde(rename = "restriction", prefix = "xs")]
    Restriction(ComplexContentRestriction),
    #[yaserde(rename = "extension", prefix = "xs")]
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

 #[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
 #[yaserde(
     rename = "restriction",
     prefix = "xs",
     namespace = "xs: http://www.w3.org/2001/XMLSchema"
 )]
 pub struct ComplexContentRestriction {
    #[yaserde(attribute)]
    pub base: Option<String>, // QName

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "openContet", prefix = "xs")]
    pub open_context: Option<OpenContent>,
 
    #[yaserde(flatten)]
    pub complex_children: ComplexChildren,

    #[yaserde(flatten)]
    pub  attribute: AttributeType,

    #[yaserde(rename = "assert", prefix = "xs")]
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

 #[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
 #[yaserde(
     rename = "extension",
     prefix = "xs",
     namespace = "xs: http://www.w3.org/2001/XMLSchema"
 )]
 pub struct ComplexContentExtension{
    #[yaserde(attribute)]
    pub base: Option<String>, // QName
 
    #[yaserde(attribute)]
    pub id: Option<String>,
 
    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
  
    #[yaserde(rename = "openContet", prefix = "xs")]
    pub open_context: Option<OpenContent>,

    #[yaserde(flatten)]
    pub attribute: AttributeType,

    #[yaserde(flatten)]
    pub complex_children: ComplexChildren,

    #[yaserde(rename = "assert", prefix = "xs")]
    pub assert: Vec<Assert>,
 }

/**
 * <simpleContent
 *   id = ID
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (restriction | extension))
 * </simpleContent> 
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "simpleContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub value: SimpleContextRestrictionOrExtrension,
}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum SimpleContextRestrictionOrExtrension {
    #[default]
    Undefined,
    #[yaserde(rename = "restriction", prefix = "xs")]
    Restriction(SimpleContentRestriction),
    #[yaserde(rename = "extension", prefix = "xs")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "restriction",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleContentRestriction {
    #[yaserde(attribute)]
    pub base: Option<String>, // QName

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType", prefix = "xs")]
    pub simple_type: Option<SimpleType>,

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

    #[yaserde(flatten)]
    pub attribute: AttributeType,

    #[yaserde(rename = "assert", prefix = "xs")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "extension",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct SimpleContentExtension{
    #[yaserde(attribute)]
    pub base: Option<String>, // QName

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten)]
    pub attribute: AttributeType,

    #[yaserde(rename = "assert", prefix = "xs")]
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
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "openContent",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct OpenContent {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mode: Option<Mode>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "any")]
    pub any: Option<Any>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Mode {
    #[default]
    #[yaserde(rename = "none")]
    None,

    #[yaserde(rename = "interleave")]
    Interleave,

    #[yaserde(rename = "suffix")]
    Suffix,
}
