use std::ffi::c_float;
use crate::xsd::default_fn::*;

use yaserde::*;
use crate::xsd::{
    simple_type::SimpleType,
    complex_type::ComplexType,
    any::Any,
    annotation::Annotation
};


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "selector",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Selector {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "xpath")]
    pub xpath: String,

    #[yaserde(attribute, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "field",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Field {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "xpath")]
    pub xpath: String,

    #[yaserde(attribute, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "unique",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Unique {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute)]
    pub reference: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "key",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Key {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute)]
    pub reference: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "keyref",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct KeyRef {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute)]
    pub reference: Option<String>,

    #[yaserde(attribute, rename = "refer")]
    pub refer: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "alternative",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Alternative {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub test: Option<String>,

    #[yaserde(attribute,rename = "type")]
    pub type_v: Option<String>,

    #[yaserde(attribute, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType", prefix = "xs")]
    pub simple_type:Option<SimpleType>,
    
    #[yaserde(rename = "complexType", prefix = "xs")]
    pub complex_type:Option<ComplexType>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "assert",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Assert {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub test: Option<String>, // XPath 表达式

    #[yaserde(attribute, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Form {
    #[default]
    #[yaserde(rename = "qualified")]
    Qualified,
    #[yaserde(rename = "unqualified")]
    Unqualified,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum ProcessContents {
    #[yaserde(rename = "lax")]
    Lax,

    #[yaserde(rename = "skip")]
    Skip,

    #[default]
    #[yaserde(rename = "strict")]
    Strict,
}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "restriction",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Restriction {
    #[yaserde(attribute)]
    pub base: Option<String>, // QName

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType")]
    pub simple_type: Vec<SimpleType>,

    #[yaserde(rename = "minExclusive")]
    pub min_exclusive: Vec<MinExclusive>,

    #[yaserde(rename = "minInclusive")]
    pub min_inclusive: Vec<MinInclusive>,

    #[yaserde(rename = "maxExclusive")]
    pub max_exclusive: Vec<MaxExclusive>,

    #[yaserde(rename = "maxInclusive")]
    pub max_inclusive: Vec<MaxInclusive>,

    #[yaserde(rename = "totalDigits")]
    pub total_digits: Vec<TotalDigits>,

    #[yaserde(rename = "fractionDigits")]
    pub fraction_digits: Vec<FractionDigits>,

    #[yaserde(rename = "length")]
    pub length: Vec<Length>,

    #[yaserde(rename = "minLength")]
    pub min_length: Vec<MinLength>,

    #[yaserde(rename = "maxLength")]
    pub max_length: Vec<MaxLength>,

    #[yaserde(rename = "enumeration")]
    pub enumeration: Vec<Enumeration>,

    #[yaserde(rename = "whiteSpace")]
    pub white_space: Vec<WhiteSpace>,

    #[yaserde(rename = "pattern")]
    pub pattern: Vec<Pattern>,

    #[yaserde(rename = "assertion")]
    pub assertion: Vec<Assertion>,

    #[yaserde(rename = "explicitTimezone")]
    pub explicit_timezone: Vec<ExplicitTimezone>,

    #[yaserde(rename = "any")]
    pub any: Vec<Any>,
}



#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minExclusive",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MinExclusive {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<f32>, // anySimpleType，使用 String 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minInclusive",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MinInclusive {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<f32>, // anySimpleType，使用 String 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxExclusive",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MaxExclusive {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<f32>, // anySimpleType，使用 String 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxInclusive",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MaxInclusive {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<f32>, // anySimpleType，使用 String 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "totalDigits",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct TotalDigits {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<u32>, // positiveInteger，使用 u32 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "fractionDigits",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct FractionDigits {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<u32>, // nonNegativeInteger，使用 u32 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "length",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Length {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<u32>, // nonNegativeInteger，使用 u32 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minLength",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MinLength {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<u32>, // nonNegativeInteger，使用 u32 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxLength",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct MaxLength {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<u32>, // nonNegativeInteger，使用 u32 处理

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "enumeration",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Enumeration {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "whiteSpace",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct WhiteSpace {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,// 默认值为 false
    
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "pattern",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Pattern {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "assertion",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Assertion {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute)]
    pub test: Option<String>, 
    
    #[yaserde(attribute,rename="xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>, 

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "explicitTimezone",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ExplicitTimezone {
    #[yaserde(attribute,default = "default_false")]
    pub fixed: bool,
    
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
