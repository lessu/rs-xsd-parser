pub mod attribute;
pub mod datamodel_map;
pub mod types;

use datamodel_map::XsdDataModel;

use crate::xsd::{attribute::{Attribute, AttributeGroup, RefAttributeGroup, AttributeType}, element::{Element, TypeComponent}};
impl TypeComponent{
    pub fn flatten<'a>(&self, _xsd:&'a XsdDataModel) -> Self {
        match self {
            TypeComponent::None => self.clone(),
            TypeComponent::ComplexType(_ct) => {
                self.clone()
            },
            TypeComponent::SimpleType(_st) => {
                self.clone()
            }
        }
    }
}

impl Element {
    pub fn flatten<'a>(&self, xsd:&'a XsdDataModel) -> Self {
        let mut new = self.clone();
        if new.type_v.is_some() {
            let type_name = new.type_v.as_ref().unwrap();
            if let Some(ct) = xsd.complex_type.get(type_name) {
                let t = (*ct).clone();
                new.type_component = TypeComponent::ComplexType(t);
            }else if let Some(st) = xsd.simple_type.get(type_name) {
                let t = (*st).clone();
                new.type_component = TypeComponent::SimpleType(t);
            }
        }
        
        // flatten type
        new.type_component.flatten(xsd);

        new
    }
}
