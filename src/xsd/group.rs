use yaserde::*;
use crate::xsd::{
  rust_types_mapping::RustTypesMapping, sequence::Sequence, XsdContext,
};

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Group {
  #[yaserde(attribute)]
  pub name: Option<String>,
  #[yaserde(attribute, rename = "ref")]
  pub reference: Option<String>,
  #[yaserde()]
  pub sequence: Option<Sequence>,
}


// #[cfg(test)]
// mod tests {
//   use super::*;

//   use yaserde::de::from_str;

//   #[test]
//   fn check_group_implementation() {
//     let xml = r#"
//         <group name="groupthing">
//           <sequence>
//             <element name="CX_X" type="asdfg"/>
//             <element name="CY_X" type="asdfg"/>
//           </sequence>
//         </group>
//     "#;

//     let group: Group = from_str(xml).unwrap();

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let implementation = format!("{}", group.implement(&TokenStream::new(), &None, &context));

//     assert_eq!(implementation, "# [derive (Clone , Debug , Default , PartialEq , serde :: Deserialize , serde :: Serialize)] \
// pub struct Groupthing { \
// # [yaserde (rename = \"CX_X\")] pub cx_x : xml_schema_types :: Asdfg , \
// # [yaserde (rename = \"CY_X\")] pub cy_x : xml_schema_types :: Asdfg , }");
//   }

//   #[test]
//   fn check_group_ref() {
//     let xml = r#"<group ref="bla:groupthing" />"#;

//     let group: Group = from_str(xml).unwrap();

//     let context =
//       XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
//         .unwrap();

//     let type_implementation = format!("{}", group.get_type_implementation(&context, &None));

//     assert_eq!(type_implementation, "Groupthing");
//   }
// }
