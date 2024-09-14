use yaserde::*;
use crate::xsd::{
    simple_type::SimpleType,
    complex_type::ComplexType,
};

use crate::xsd::annotation::Annotation;

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
    pub type_: Option<String>,

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
    #[default]
    #[yaserde(rename = "lax")]
    Lax,

    #[yaserde(rename = "skip")]
    Skip,

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
    pub min_exclusive: Vec<String>,

    #[yaserde(rename = "minInclusive")]
    pub min_inclusive: Vec<String>,

    #[yaserde(rename = "maxExclusive")]
    pub max_exclusive: Vec<String>,

    #[yaserde(rename = "maxInclusive")]
    pub max_inclusive: Vec<String>,

    #[yaserde(rename = "totalDigits")]
    pub total_digits: Vec<u32>,

    #[yaserde(rename = "fractionDigits")]
    pub fraction_digits: Vec<String>,

    #[yaserde(rename = "length")]
    pub length: Vec<u32>,

    #[yaserde(rename = "minLength")]
    pub min_length: Vec<u32>,

    #[yaserde(rename = "maxLength")]
    pub max_length: Vec<u32>,

    #[yaserde(rename = "enumeration")]
    pub enumeration: Vec<String>,

    #[yaserde(rename = "whiteSpace")]
    pub white_space: Vec<String>,

    #[yaserde(rename = "pattern")]
    pub pattern: Vec<String>,

    #[yaserde(rename = "assertion")]
    pub assertion: Vec<String>,

    #[yaserde(rename = "explicitTimezone")]
    pub explicit_timezone: Vec<String>,

    #[yaserde(rename = "any")]
    pub any: Vec<String>,
}
