pub mod plant;

use crate::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use plant::Plant;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Plant,
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Type::*;

        let ty = match s {
            "plant" | "plants" => Plant,
            _ => return Err("invalid data type".into()),
        };

        Ok(ty)
    }
}
