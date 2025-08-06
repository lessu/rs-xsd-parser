use serde::Deserialize;

use crate::xsd::annotation::Annotation;
/**
 * <import
 *  id = ID
 *  namespace = anyURI
 *  schemaLocation = anyURI
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?)
 * </import>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@namespace")]
    pub namespace: Option<String>,

    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<String>,

    #[serde(default)]
    pub annotations: Vec<Annotation>
}
