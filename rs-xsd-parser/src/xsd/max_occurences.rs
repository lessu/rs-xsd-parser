use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub enum MaxOccurences {
    Unbounded,
    Number {
        value: u32,
    },
}

impl Default for MaxOccurences {
    fn default() -> Self {
        MaxOccurences::Number{ value: 1 }
    }
}

impl<'de> Deserialize<'de> for MaxOccurences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        if s == "unbounded" {
            Ok(MaxOccurences::Unbounded)
        } else {
            let number = s.parse::<u32>().map_err(|e| serde::de::Error::custom(e.to_string()))?;
            Ok(MaxOccurences::Number { value: number })
        }
    }
}

