use yaserde::*;

use crate::xsd::{
    annotation::Annotation,
    type_def::Restriction,
    type_def::Extension,
    any::Any
};


#[derive(Clone, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub enum RestrictionOrExtrension {
    #[yaserde(rename = "restriction", prefix = "xs")]
    Restriction(Restriction),
    #[yaserde(rename = "extension", prefix = "xs")]
    Extension(Extension),
}
impl Default for RestrictionOrExtrension {
    fn default() -> Self {
        RestrictionOrExtrension::Restriction(Restriction::default())
    }
}

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
    pub value: RestrictionOrExtrension,

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
    pub value: RestrictionOrExtrension,
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
