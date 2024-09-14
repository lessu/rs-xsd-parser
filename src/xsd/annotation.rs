use yaserde::*;

use crate::xsd::any::Any;

/**
 * <annotation
 *  id = ID
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (appinfo | documentation)*
 * </annotation>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "annotation",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Annotation {
    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(rename = "appinfo")]
    pub appinfo: Vec<AppInfo>,

    #[yaserde(rename = "documentation")]
    pub documentation: Vec<Documentation>,
}

/**
 * <appinfo
 *  source = anyURI
 *  {any attributes with non-schema namespace . . .}>
 *    Content: ({any})*
 * </appinfo>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "appinfo",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AppInfo {
    #[yaserde(attribute)]
    pub source: Option<String>, // anyURI

    #[yaserde(rename = "any")]
    pub content: Vec<Any>,
}
/**
 * <documentation
 *  source = anyURI
 *  xml:lang = language
 *  {any attributes with non-schema namespace . . .}>
 *    Content: ({any})*
 * </documentation>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "documentation",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Documentation {
    #[yaserde(attribute)]
    pub source: Option<String>, // anyURI

    #[yaserde(attribute, rename = "xml:lang")]
    pub xml_lang: Option<String>, // language

    #[yaserde(rename = "any")]
    pub content: Vec<Any>,
}