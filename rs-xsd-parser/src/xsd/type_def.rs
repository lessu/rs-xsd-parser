use crate::xsd::default_fn::*;

use yaserde::*;
use crate::xsd::{
    annotation::Annotation,
    sequence::{All, Choice, Sequence},
    element::TypeComponent,
    group::Group
};

use super::{common_type::QName, types::Types};


/**
 * <selector
 *   id = ID
 *   xpath = a subset of XPath expression, see below
 *   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </selector>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "selector",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Selector {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "xpath")]
    pub xpath: String,

    #[yaserde(attribute = true, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
/**
 * <field
 *   id = ID
 *   xpath = a subset of XPath expression, see below
 *   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </field>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "field",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Field {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "xpath")]
    pub xpath: String,

    #[yaserde(attribute = true, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

/**
 * <unique
 *   id = ID
 *   name = NCName
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (selector, field+)?)
 * </unique>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "unique",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Unique {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true)]
    pub reference: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}

/**
 * <key
 *   id = ID
 *   name = NCName
 *   ref = QName
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (selector, field+)?)
 * </key>
 */

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "key",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Key {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true)]
    pub reference: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}

/**
 * <keyref
 *   id = ID
 *   name = NCName
 *   ref = QName
 *   refer = QName
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (selector, field+)?)
 * </keyref>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "keyref",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct KeyRef {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub name: Option<String>,

    #[yaserde(attribute = true)]
    pub reference: Option<String>,

    #[yaserde(attribute = true, rename = "refer")]
    pub refer: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "selector", prefix = "xs")]
    pub selector: Option<Selector>,

    #[yaserde(rename = "field", prefix = "xs")]
    pub fields: Vec<Field>,
}
/**
 * <alternative
 *   id = ID
 *   test = an XPath expression
 *   type = QName
 *   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (simpleType | complexType)?)
 * </alternative>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "alternative",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Alternative {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub test: Option<String>,

    #[yaserde(attribute = true,rename = "type")]
    pub type_v: Option<QName<Types>>,

    #[yaserde(attribute = true, rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(flatten = true)]
    pub type_component:TypeComponent
}
/**
 * <assert
 *   id = ID
 *   test = an XPath expression
 *   xpathDefaultNamespace = (anyURI | (##defaultNamespace | ##targetNamespace | ##local))
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?)
 * </assert>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "assert",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Assert {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub test: Option<String>, // XPath

    #[yaserde(attribute = true, rename = "xpathDefaultNamespace")]
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
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub enum ComplexChildren{
    #[default]
    None,

    #[yaserde(rename = "group", prefix = "xs")]
    Group(Group),
    
    #[yaserde(rename = "all", prefix = "xs")]
    All(All),
    
    #[yaserde(rename = "choice", prefix = "xs")]
    Choice(Choice),

    #[yaserde(rename = "sequence", prefix = "xs")]
    Sequence(Sequence)
}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minExclusive",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MinExclusive {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<f32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minInclusive",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MinInclusive {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<f32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxExclusive",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MaxExclusive {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<f32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxInclusive",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MaxInclusive {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<f32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "totalDigits",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct TotalDigits {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<u32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "fractionDigits",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct FractionDigits {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<u32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "length",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Length {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<u32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "minLength",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MinLength {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<u32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "maxLength",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct MaxLength {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<u32>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "enumeration",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Enumeration {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "whiteSpace",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct WhiteSpace {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,
    
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "pattern",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Pattern {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "assertion",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Assertion {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,

    #[yaserde(attribute = true)]
    pub test: Option<String>, 
    
    #[yaserde(attribute = true,rename="xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>, 

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}


#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "explicitTimezone",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct ExplicitTimezone {
    #[yaserde(attribute = true,default = "default_false")]
    pub fixed: bool,
    
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true)]
    pub value: Option<String>,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,
}
