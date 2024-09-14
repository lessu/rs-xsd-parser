use yaserde::*;
use crate::xsd::default_fn::*;
use crate::xsd::{
    group::Group,
    attribute::Attribute,
    simple_content::SimpleContent,
    complex_content::ComplexContent,
    annotation::Annotation,
    any_attribute::AnyAttribute,
    attribute_group::AttributeGroup,
    type_def::Assert, open_content::OpenContent,
    sequence::{All, Choice, Sequence},
    simple_type::SimpleType
};
/**
 * <complexType
 *  abstract = boolean : false
 *  block = (#all | List of (extension | restriction))
 *  final = (#all | List of (extension | restriction))
 *  id = ID
 *  mixed = boolean
 *  name = NCName
 *  defaultAttributesApply = boolean : true
 *  {any attributes with non-schema namespace . . .}>
 *    Content: (annotation?, (simpleContent | complexContent | (openContent?, (group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?), assert*)))
 * </complexType>
 */
#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "complexType",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexType {
    #[yaserde(attribute,default = "default_false")]
    pub abstract_v: bool,

    #[yaserde(attribute, rename = "block")]
    pub block: Option<String>,

    #[yaserde(attribute, rename = "final")]
    pub final_v: Option<String>,

    #[yaserde(attribute)]
    pub id: Option<String>,

    #[yaserde(attribute)]
    pub mixed: Option<bool>,

    #[yaserde(attribute)]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "defaultAttributesApply", default = "default_true")]
    pub default_attributes_apply: bool,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "simpleType", prefix = "xs")]
    simple_type: Vec<SimpleType>, // ( SimpleType | ComplexType )? due to nest referencing, we can't use Option<TypeDefinition>

    #[yaserde(rename = "complexType", prefix = "xs")]
    complex_type: Vec<ComplexType>, // ( SimpleType | ComplexType )? due to nest referencing, we can't use Option<TypeDefinition>

    #[yaserde(rename = "simpleContent", prefix = "xs")]
    pub simple_content: Option<SimpleContent>,

    #[yaserde(rename = "complexContent", prefix = "xs")]
    pub complex_content: Option<ComplexContent>,

    #[yaserde(rename = "openContent", prefix = "xs")]
    pub open_content: Option<OpenContent>,

    #[yaserde(rename = "group", prefix = "xs")]
    pub group: Option<Group>,

    #[yaserde(rename = "all", prefix = "xs")]
    pub all: Option<All>,

    #[yaserde(rename = "choice", prefix = "xs")]
    pub choice: Option<Choice>,

    #[yaserde(rename = "sequence", prefix = "xs")]
    pub sequence: Option<Sequence>,

    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,

    #[yaserde(rename = "AttributeGroup")]
    pub attribute_groups: Vec<AttributeGroup>,

    #[yaserde(rename = "anyAttribute")]
    pub any_attribute: Option<AnyAttribute>,

    #[yaserde(rename = "assert")]
    pub asserts: Vec<Assert>,
}
