// use std::str::FromStr;

// use crate::xsd::{group::Group, type_def::ComplexChildren, types::ComplexType};

// use super::datamodel_map::XsdDataModel;

// pub struct FlattenResult<T>{
//     pub result:Result<T,String>,
//     pub warns:Vec<String>
// }
// impl ComplexType {
//     fn expand_group(&mut self, group:&Group) {

//     }
//     pub fn flatten(&self, _xsd:&XsdDataModel) -> FlattenResult<Self>{
//         match self.complex_children{
//             ComplexChildren::Group(group) => {
//                 let mut new_type = self.clone();
//                 new_type.expand_group(&group);
//                 FlattenResult{
//                     result: Ok(new_type),
//                     warns : Vec::new()
//                 }
//             },
//             ComplexChildren::All(all) => {
//                 todo!()
//             },
//             ComplexChildren::Sequence(sequence) => {
//                 todo!()
//             },
//             ComplexChildren::Choice(choice) => {
//                 todo!()
//                 FlattenResult{
//                     result: Err(String::from_str("complex type with choice is not supported").unwrap()),
//                     warns : Vec::new()
//                 }
//             },
//             _=> {
//                 FlattenResult{
//                     result: Ok(self.clone()),
//                     warns : Vec::new()
//                 }
//             }
//         }
//     }
// }