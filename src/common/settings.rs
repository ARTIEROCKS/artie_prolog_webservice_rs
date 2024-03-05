use serde::{Deserialize, Serialize};
use loco_rs::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    pub mongodb: Option<Mongodb>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Mongodb {
    pub connection_string: String,
    pub username: String,
    pub passwd: String,
    pub db: String
} 

impl Settings {
    pub fn from_json(value: &serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(value.clone())?)
    }
}