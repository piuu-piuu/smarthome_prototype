use diesel::prelude::*;
use std::collections::{BTreeMap, HashSet};

#[derive(Queryable)]
pub struct SmartHouse<'a> {
    pub name: &'a str,
    pub rooms: BTreeMap<&'a str, HashSet<&'a str>>,
}
