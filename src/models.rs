use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartHouse<'a> {
    pub name: &'a str,
    pub rooms: BTreeMap<&'a str, HashSet<&'a str>>,
}
