use bevy::prelude::*;
use bevy_assets_toml::Toml;
use data::*;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone)]
pub struct DataHandleCollection {
    pub inner: Vec<Handle<Toml>>,
}

#[derive(Debug, Default)]
pub struct PlantData {
    pub inner: HashSet<Plant>,
}

impl Deref for PlantData {
    type Target = HashSet<Plant>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PlantData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
