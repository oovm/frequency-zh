use serde_json::Value;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ops::AddAssign;

pub enum ErrorHere {
    JSON,
    Optional,
    IO,
}

impl From<serde_json::error::Error> for ErrorHere {
    fn from(_: serde_json::error::Error) -> Self {
        ErrorHere::JSON
    }
}

impl From<std::option::NoneError> for ErrorHere {
    fn from(_: std::option::NoneError) -> Self {
        ErrorHere::Optional
    }
}

impl From<std::io::Error> for ErrorHere {
    fn from(_: std::io::Error) -> Self {
        ErrorHere::IO
    }
}

#[derive(Debug)]
pub struct CharMap(BTreeMap<char, usize>);

impl Default for CharMap {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddAssign<CharMap> for CharMap {
    fn add_assign(&mut self, rhs: CharMap) {
        for (c, n) in rhs.0 {
            *self.0.entry(c).or_insert(0) += n;
        }
    }
}

impl CharMap {
    pub fn entry(&mut self, key: char) -> Entry<char, usize> {
        self.0.entry(key)
    }
    pub fn to_json(&self) -> Value {
        match serde_json::to_value(&self.0) {
            Ok(o) => o,
            Err(_) => Value::Null,
        }
    }
}
