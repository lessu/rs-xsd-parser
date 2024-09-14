use std::collections::BTreeMap;

use crate::xsd::{attribute::{Attribute, AttributeGroup}, element::Element, group::Group, types::{ComplexType, SimpleType}, Xsd};

#[derive(Debug)]
pub struct XsdDataModel<'a> {
    pub elements: BTreeMap<String,&'a Element>,
    pub simple_type: BTreeMap<String,&'a SimpleType>,
    pub complex_type: BTreeMap<String,&'a ComplexType>,
    pub attribute: BTreeMap<String,&'a Attribute>,
    pub attribute_group: BTreeMap<String,&'a AttributeGroup>,
    pub group: BTreeMap<String,&'a Group>,
}

impl<'a> XsdDataModel<'a> {
    pub fn build(xsd: &'a Xsd) -> Self {
        let mut elements = BTreeMap::new();
        let mut simple_type = BTreeMap::new();
        let mut complex_type = BTreeMap::new();
        let mut attribute = BTreeMap::new();
        let mut attribute_group = BTreeMap::new();
        let mut group = BTreeMap::new();

        let mut target_ns_prefix = "";
        if xsd.schema.target_namespace.is_some() {
            let target_namespace = xsd.schema.target_namespace.as_ref().unwrap();
            for (key,value) in xsd.namespace.iter() {
                if value == target_namespace {
                    target_ns_prefix = key;
                }
            }
        }
        // build map
        macro_rules! build_map {
            ($map_name:ident) => {
                xsd.schema.$map_name.iter().for_each(|e| {
                    if e.name.is_some(){
                        // 怎么改
                        $map_name.insert(format!("{}:{}",target_ns_prefix,e.name.as_ref().unwrap()), e);
                    }
                });
            }
        }
        build_map!(elements);
        build_map!(simple_type);
        build_map!(complex_type);
        build_map!(attribute);
        build_map!(attribute_group);
        build_map!(group);

        Self {
            elements,
            simple_type,
            complex_type,
            attribute,
            attribute_group,
            group,
        }
    }
    
}