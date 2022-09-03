use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SmartHouse {
    pub name: String,
    pub rooms: BTreeMap<String, HashSet<String>>,
}
