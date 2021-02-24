use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_assets_toml::Toml;
use data::Type;
use serde::Deserialize;
use std::str::FromStr;

use crate::kind::{DataHandleCollection, PlantData};
use crate::tracing;

#[tracing::instrument(skip(commands, asset_server))]
pub(crate) fn load(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let handles = match asset_server.load_folder("data") {
        Ok(data) => data,
        Err(error) => {
            error!(%error, "failed to load data");
            vec![]
        }
    };

    let inner = handles.into_iter().map(|handle| handle.typed()).collect();

    commands.insert_resource(DataHandleCollection { inner });
}

#[tracing::instrument(skip(asset_server))]
pub(crate) fn setup(
    mut app_exit_events: ResMut<Events<AppExit>>,
    asset_server: Res<AssetServer>,
    datum: Res<DataHandleCollection>,
    mut plants: ResMut<PlantData>,
    toml: Res<Assets<Toml>>,
) {
    let mut exit = |error: String| {
        error!(%error, "failed to set-up data assets");
        app_exit_events.send(AppExit);
    };

    match asset_server.get_group_load_state(datum.inner.clone().iter().map(|h| h.id)) {
        LoadState::Loaded => {}
        LoadState::Failed => return exit("data assets failed to load".to_owned()),
        _ => return,
    }

    for handle in &datum.inner {
        let value = match toml.get(handle) {
            Some(toml) => &toml.value,
            None => return exit("failed to load toml file".to_owned()),
        };

        let ty = match value.get("type") {
            Some(value) if value.is_str() => match Type::from_str(value.as_str().unwrap()) {
                Ok(ty) => ty,
                Err(error) => return exit(format!("invalid type value: {}", error)),
            },
            _ => return exit("missing type field".to_owned()),
        };

        match ty {
            Type::Plant => plants.insert(match de(value) {
                Ok(v) => v,
                Err(err) => return exit(format!("invalid plant data: {}", err)),
            }),
        };
    }
}

fn de<'de, D: Deserialize<'de>>(value: &'de toml::Value) -> Result<D, String> {
    value.clone().try_into().map_err(|err| err.to_string())
}
