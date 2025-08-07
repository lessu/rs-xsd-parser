use yaserde::*;
use crate::xsd::default_fn::*;

use crate::xsd::{
    annotation::Annotation,
    any::Any,
    element::Element,
    group::Group,
    max_occurences::MaxOccurences
};

/**
 * <all
 *   id = ID
 *   maxOccurs = (0 | 1) : 1
 *   minOccurs = (0 | 1) : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | any | group)*)
 * </all>
 */
#[derive(Clone, Default, Debug, YaDeserialize)]
#[yaserde(
    rename = "all",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct All {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs", default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs", default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element", prefix = "xs")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "any", prefix = "xs")]
    pub any: Vec<Any>,

    #[yaserde(rename = "group", prefix = "xs")]
    pub groups: Vec<Group>,
}
/**
 * <choice
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | group | choice | sequence | any)*)
 * </choice>
 */
#[derive(Clone, Default, Debug, YaDeserialize)]
#[yaserde(
    rename = "choice",
    prefix = "xs",
    namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
)]
pub struct Choice {
    #[yaserde(attribute = true)]
    pub id: Option<String>,

    #[yaserde(attribute = true, rename = "maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    #[yaserde(attribute = true, rename = "minOccurs",default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    #[yaserde(rename = "element", prefix = "xs")]
    pub elements: Vec<Element>,

    #[yaserde(rename = "group", prefix = "xs")]
    pub groups: Vec<Group>,

    #[yaserde(rename = "choice", prefix = "xs")]
    pub choices: Vec<Choice>,

    #[yaserde(rename = "sequence", prefix = "xs")]
    pub sequences: Vec<Sequence>,

    #[yaserde(rename = "any", prefix = "xs")]
    pub any: Vec<Any>,
}

// We need sequence, it is not supported by yaserde, so we need to implement it manually
/**
 * <sequence
 *   id = ID
 *   maxOccurs = (nonNegativeInteger | unbounded)  : 1
 *   minOccurs = nonNegativeInteger : 1
 *   {any attributes with non-schema namespace . . .}>
 *     Content: (annotation?, (element | group | choice | sequence | any)*)
 * </sequence>
 */
#[derive(Clone, Default, Debug)]
// #[yaserde(
//     rename = "sequence",
//     prefix = "xs",
//     namespaces = {"xs" = "http://www.w3.org/2001/XMLSchema" }
// )]
pub struct Sequence {
    // #[yaserde(attribute = true)]
    pub id: Option<String>,

    // #[yaserde(attribute = true, rename = "maxOccurs",default = "default_max_occurs")]
    pub max_occurs: MaxOccurences,

    // #[yaserde(attribute = true, rename = "minOccurs",default = "default_min_occurs")]
    pub min_occurs: MaxOccurences,

    // #[yaserde(rename = "annotation", prefix = "xs")]
    pub annotation: Option<Annotation>,

    // #[yaserde(flatten = true)]
    pub contents: Vec<SequenceContent>
}

#[derive(Clone, Debug)]
pub enum SequenceContent {
    Element(Element),
    Group(Group),
    Choice(Choice),
    Sequence(Sequence),
    Any(Any),
}

impl YaDeserialize for Sequence {
    fn deserialize<R: std::io::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let mut sequence = Sequence::default();
        let mut level = 0;
    
        while let Ok(event) = reader.peek() {
            match event {
                xml::reader::XmlEvent::StartElement { name, attributes, namespace: _ } => {
                    if level == 0 {
                        if name.local_name == "sequence" {
                            level = level + 1;
                            for a in attributes{
                                if a.name.local_name == "id" {
                                    sequence.id = Some(a.value.clone());
                                } else if a.name.local_name == "maxOccurs" {
                                    sequence.max_occurs = MaxOccurences::from_str(&a.value).unwrap();
                                } else if a.name.local_name == "minOccurs" {
                                    sequence.min_occurs = MaxOccurences::from_str(&a.value).unwrap();
                                }
                            }
                        }else{
                            return Err(format!("Unexpected element: {}", name.local_name));
                        }
                    }else if level == 1 {
                            
                        let local_name = &name.local_name;
                        if local_name == "annotation" {
                            let annotation: Annotation = Annotation::deserialize(reader)?;
                            sequence.annotation = Some(annotation);
                        } else if local_name == "element" {
                            let element: Element = Element::deserialize(reader)?;
                            sequence.contents.push(SequenceContent::Element(element));
                        } else if local_name == "group" {
                            let group: Group = Group::deserialize(reader)?;
                            sequence.contents.push(SequenceContent::Group(group));
                        } else if local_name == "choice" {
                            let choice: Choice = Choice::deserialize(reader)?;
                            sequence.contents.push(SequenceContent::Choice(choice));
                        } else if local_name == "sequence" {
                            let sequence_content: Sequence = Sequence::deserialize(reader)?;
                            sequence.contents.push(SequenceContent::Sequence(sequence_content));
                        } else if local_name == "any" {
                            let any: Any = Any::deserialize(reader)?;
                            sequence.contents.push(SequenceContent::Any(any));
                        } else {
                            // pass unexpected element
                        }
                    }else{
                        unreachable!()
                    }
                }
                xml::reader::XmlEvent::EndElement { .. } => {
                    level = level - 1;
                    if level == 0 {
                        break;
                    }else{
                        return Err(format!("Unexpected end element"));
                    }
                }
                _ => {
                    // pass
                }
            }
            reader.next_event()?;
        }
        Ok(sequence)
    }
}