use crate::xsd::default_fn::*;

use serde::Deserialize;
use serde::*;
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@xpath")]
    pub xpath: String,

    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[serde()]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@xpath")]
    pub xpath: String,

    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[serde()]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unique {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@reference")]
    pub reference: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "selector")]
    pub selector: Option<Selector>,

    #[serde(rename = "field")]
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

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@reference")]
    pub reference: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "selector")]
    pub selector: Option<Selector>,

    #[serde(rename = "field")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyRef {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@reference")]
    pub reference: Option<String>,

    #[serde(rename = "@refer")]
    pub refer: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(rename = "selector")]
    pub selector: Option<Selector>,

    #[serde(rename = "field")]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@test")]
    pub test: Option<String>,

    #[serde(rename = "@type")]
    pub type_v: Option<QName<Types>>,

    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,

    #[serde(flatten)]
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
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assert {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@test")]
    pub test: Option<String>, // XPath

    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Form {
    #[default]
    Qualified,
    Unqualified,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProcessContents {
    Lax,
    Skip,
    #[default]
    Strict,
}


#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComplexChildren{
    #[default]
    None,
    Group(Group),
    All(All),
    Choice(Choice),
    Sequence(Sequence)
}


#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MinExclusive {
    #[serde(rename = "@fixed",default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<f32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MinInclusive {
    #[serde(rename = "@fixed",default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<f32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MaxExclusive {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<f32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MaxInclusive {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<f32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct TotalDigits {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<u32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct FractionDigits {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<u32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Length {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<u32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MinLength {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<u32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct MaxLength {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<u32>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Enumeration {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct WhiteSpace {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,
    
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Pattern {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Assertion {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,

    #[serde(rename = "@test")]
    pub test: Option<String>, 
    
    #[serde(rename = "@xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<String>, 

    #[serde()]
    pub annotation: Option<Annotation>,
}


#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct ExplicitTimezone {
    #[serde(rename = "fixed", default = "default_false")]
    pub fixed: bool,
    
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde()]
    pub annotation: Option<Annotation>,
}
