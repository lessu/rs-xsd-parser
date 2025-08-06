use serde::Deserialize;

/**
 * <annotation
 *  id = ID
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (appinfo | documentation)*
 * </annotation>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "snake_case" )]
pub struct Annotation {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "$value")]
    pub content: Vec<AnnotationContent>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde( rename_all = "snake_case" )]
pub enum AnnotationContent {
    Appinfo(Appinfo),
    Documentation(Documentation),
}
/**
 * <appinfo
 *  source = anyURI
 *  {any attributes with non-schema namespace . . .}>
 *    Content: ({any})*
 * </appinfo>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "snake_case" )]
pub struct Appinfo {
    #[serde(rename = "@source")]
    pub source: Option<String>, // anyURI

    #[serde(rename = "$value")]
    pub text: String,
}
/**
 * <documentation
 *  source = anyURI
 *  xml:lang = language
 *  {any attributes with non-schema namespace . . .}>
 *    Content: ({any})*
 * </documentation>
 */
#[derive(Clone, Default, Debug, Deserialize)]
#[serde( rename_all = "snake_case" )]
pub struct Documentation {
    #[serde(rename = "@source")]
    pub source: Option<String>, // anyURI

    #[serde(rename = "@lang")]
    pub lang: Option<String>, // language

    #[serde(rename = "$value")]
    pub text: String,
}
