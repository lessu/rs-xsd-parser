use std::{fmt::{Display}, marker::PhantomData};

use serde::{Deserialize, Deserializer};

/**
 * A QName is a name with an optional namespace qualification, 
 * as defined in [XML Namespaces 1.1]. 
 * When used in connection with the XML representation of schema components 
 * or references to them, this refers to the simple type QName 
 * as defined in [XML Schema: Datatypes]
 */
#[derive(Debug,Clone,PartialEq,Default)]
pub struct QName<T> {
    __own:PhantomData<T>,
    value:String
}

impl<T> QName<T>{
    pub fn raw_value(&self) -> &str{
        &self.value
    }
}

impl<'de, T> Deserialize<'de> for QName<T>{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(QName::<T>{
            __own: PhantomData::<T>,
            value: s
        })
    }
}

impl<T> Display for QName<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

