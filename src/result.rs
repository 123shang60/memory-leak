use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Map {
    pub map: HashMap<i32, i32>,
}
