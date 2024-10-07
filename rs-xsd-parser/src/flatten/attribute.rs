use std::{error::Error, fmt::Display};

use crate::xsd::{attribute::{
    Attribute, AttributeGroup, RefAttributeGroup, RefAttributeType
}, common_type::QName};
use super::datamodel_map::XsdDataModel;

#[derive(Debug, Default, Copy, Clone)]
pub struct FlattenError;

pub struct FlattenAttribute<'a> {
    pub group: Option<&'a AttributeGroup>,
    pub attribute_list: Vec<&'a Attribute>
}

impl Display for FlattenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flatten error")
    }
}

impl RefAttributeType {
    pub fn flatten_attribute<'a>(&'a self, dm:&'a XsdDataModel) -> Result<Vec<FlattenAttribute<'a>>,FlattenError> {
        let mut ret   = Vec::new();
        let mut attrs = Vec::new();
        for attr in &self.attributes{
            attrs.push(attr);
        }
        ret.push(FlattenAttribute{
            group: None,
            attribute_list: attrs
        });

        /*
         * self.any_attributes is not used
         */
        for rag in &self.attribute_groups {
            if let Some(ag) = rag.deref(dm) {
                let ag_fl = ag.flatten_attributes(dm);
                if let Ok(res) = ag_fl {
                    for fa in res {
                        ret.push(fa);
                    }
                }else{
                    return Err(FlattenError);
                }
            }else{
                return Err(FlattenError);
            }
        }

        Ok(ret)
    }
}

impl AttributeGroup {
    pub fn flatten_attributes<'a>(&'a self, dm:&'a XsdDataModel) -> Result<Vec<FlattenAttribute<'a>>,FlattenError> {
        let mut ret   = Vec::new();
        let mut attrs = Vec::new();

        for attr in &self.attributes.attributes {
            attrs.push(attr);
        }
        ret.push(FlattenAttribute{
            group: Some(self),
            attribute_list: attrs
        });

        for rag in &self.attributes.attribute_groups {
            if let Some(ag) = rag.deref(dm) {
                let ag_fl = ag.flatten_attributes(dm);
                if let Ok(res) = ag_fl {
                    for fa in res {
                        ret.push(fa);
                    }
                }else{
                    return Err(FlattenError);
                }
            }else{
                return Err(FlattenError);
            }
        }
        Ok(ret)
    }
}

impl RefAttributeGroup {
    pub fn deref<'a>(&'a self,dm: &'a XsdDataModel) -> Option<&'a AttributeGroup>{
        if let Some(ref_name) = self.ref_v.as_ref() {
            if let Some(r) = dm.attribute_group.get(ref_name.raw_value()){
                return Some(*r);
            }
        }
        None
    }
}


pub trait AttributeTrait{ }

impl<T :AttributeTrait> QName<T>{
    pub fn dereference() -> Option<Attribute>{
        None
    }
}

impl AttributeTrait for Attribute{
}

