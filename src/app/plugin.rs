use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy_assets_toml::{Toml, TomlLoader};

use super::{AppStage as Stage, AppState as State, SystemLabel as Label};
use crate::kind::{DataHandleCollection, PlantData};
use crate::state::TileSet;
use crate::system::*;

pub struct AppPlugin;
pub struct LoadingStagePlugin;
pub struct RunningStagePlugin;
pub struct DebugPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader(TomlLoader)
            .add_asset::<Toml>()
            // Hack
            //
            // see: <https://discord.com/channels/691052431525675048/742884593551802431/813310203599912980>
            .init_resource::<DataHandleCollection>()
            .init_resource::<PlantData>()
            .init_resource::<TileSet>()
            // Exit System
            .add_system(exit_on_esc_system.system())
            // Loading
            .add_plugin(LoadingStagePlugin)
            // Running
            .add_plugin(RunningStagePlugin);
    }
}

impl Plugin for LoadingStagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state = State::Loading;

        app
            // Data Asset Loading
            .on_state_enter(
                Stage::Update,
                state,
                data::load.system().label(Label::DatumLoad),
            )
            // Tileset Asset Loading
            .on_state_enter(
                Stage::Update,
                state,
                tileset::setup.system().label(Label::TileSetSetup),
            )
            // Data Type Initialization
            .on_state_update(
                Stage::Update,
                state,
                data::setup.system().label(Label::DataTypeSetup),
            )
            // Loading State Transition
            .on_state_update(
                Stage::Update,
                state,
                state_transition::loading_to_running
                    .system()
                    .label(Label::StateLoadingToRunning),
            );
    }
}

impl Plugin for RunningStagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state = State::Running;

        app
            // Spawn Tileset
            //
            // FIXME: currently can't use `on_state_enter` because of a bug.
            //
            // see: <https://github.com/bevyengine/bevy/issues/1117>
            .init_resource::<super::OnStateEnterFix>()
            .on_state_update(
                Stage::Update,
                state,
                tileset::spawn.system().label(Label::TileSetSpawn),
            )
            // Spawn Player
            .on_state_update(
                Stage::Update,
                state,
                player::spawn.system().label(Label::PlayerSpawn),
            )
            // Spawn Camera
            .on_state_update(
                Stage::Update,
                state,
                camera::spawn.system().label(Label::CameraSpawn),
            )
            // FIXME:END
            //
            // Player Input
            .on_state_update(
                Stage::Update,
                state,
                player::input.system().label(Label::PlayerInput),
            )
            // Update TileSet
            .on_state_update(
                Stage::Update,
                state,
                tileset::update
                    .system()
                    .after(Label::PlayerInput)
                    .label(Label::TileSetUpdate),
            )
            .on_state_update(
                Stage::View,
                state,
                camera::zoom.system().label(Label::CameraZoom),
            )
            .on_state_update(
                Stage::View,
                state,
                camera::focus.system().label(Label::CameraFocus),
            )
            .on_state_update(
                Stage::View,
                state,
                render::update_translation
                    .system()
                    .after(Label::CameraFocus)
                    .label(Label::UpdateTranslation),
            )
            .on_state_update(
                Stage::View,
                state,
                render::tiles.system().label(Label::RenderTiles),
            );
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        use bevy::diagnostic::{
            EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        };
        use bevy::log::{Level, LogSettings};

        app.insert_resource(ReportExecutionOrderAmbiguities)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(EntityCountDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(LogSettings {
                level: Level::INFO,
                filter: "wgpu_core=error,bevy_ecs=info".to_string(),
            });
    }
}
