pub mod attribute;
pub mod datamodel_map;

use datamodel_map::XsdDataModel;

use crate::xsd::{attribute::{Attribute, AttributeGroup, AttributeType}, element::{Element, TypeComponent}};

impl AttributeGroup {
    pub fn is_ref(&self) -> bool{
        self.ref_v.is_some()
    }
    pub fn flatten_attributes<'a>(&'a self, xsd:&'a XsdDataModel) -> Vec<&'a Attribute>{

        if self.is_ref() {
            let ref_name = self.ref_v.as_ref().unwrap();
            if let Some(ag) = xsd.attribute_group.get(ref_name) {
                ag.flatten_attributes(xsd)
            } else {
                Vec::new()
            }
        }else{
            let mut attributes = Vec::new();

            for attr in &self.attributes.as_ref().unwrap().attributes {
                attributes.push(attr);
            }
            for ag in &self.attributes.as_ref().unwrap().attribute_groups {
                let child_attributes = ag.flatten_attributes(xsd);
                for attr in child_attributes {
                    attributes.push(attr);
                }
            }
            attributes
        }
    }
}

pub fn flattern_attribute<'a>(attr:&'a AttributeType, datamodel: &'a XsdDataModel) -> Vec<&'a Attribute>{
    let mut attributes = Vec::new();
    for attr in &attr.attributes {
        attributes.push(attr);
    }

    for ag in &attr.attribute_groups {
        let child_attributes = ag.flatten_attributes(datamodel);
        for attr in child_attributes {
            attributes.push(attr);
        }
    }
    attributes
}

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