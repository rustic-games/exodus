mod error;
pub mod property;
mod ty;

pub use error::*;
pub use ty::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Data {
    Plant(Plant),
}

impl Data {
    pub fn name(&self) -> &str {
        use Data::*;

        match self {
            Plant(v) => &v.name,
        }
    }
}
