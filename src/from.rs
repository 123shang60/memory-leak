use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct create {
    pub username: String,
    pub password: String,
}
