use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::property::{MaxHeight, Name};
use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Plant {
    pub id: Uuid,
    pub name: Name,
    pub habit: Habit,
    pub max_height: MaxHeight,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Habit {
    /// A tree typically has many secondary branches supported clear of the
    /// ground by the trunk. This trunk typically contains woody tissue for
    /// strength, and vascular tissue to carry materials from one part of the
    /// tree to another. For most trees it is surrounded by a layer of bark
    /// which serves as a protective barrier. Below the ground, the roots branch
    /// and spread out widely.
    Tree,

    /// A shrub or bush is a small- to medium-sized perennial woody plant.
    Shrub,

    /// The Poaceae are the most economically important plant family, providing
    /// staple foods from domesticated cereal crops such as maize, wheat, rice,
    /// barley, and millet as well as feed for meat-producing animals. They
    /// provide, through direct human consumption, just over one-half (51%) of
    /// all dietary energy.
    Grass,

    /// A vine is any plant with a growth habit of trailing or scandent (that
    /// is, climbing) stems, lianas or runners.
    Vine,

    /// In general use, herbs are plants with savory or aromatic properties that
    /// are used for flavoring and garnishing food, for medicinal purposes, or
    /// for fragrances; excluding vegetables and other plants consumed for
    /// macronutrients.
    Herb,
}

impl FromStr for Habit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Habit::*;

        let ty = match s {
            "tree" => Tree,
            "shrub" => Shrub,
            "grass" => Grass,
            "vine" => Vine,
            "herb" => Herb,
            _ => return Err(Error::Plant("invalid habit".to_owned())),
        };

        Ok(ty)
    }
}
