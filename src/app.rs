mod label;
mod plugin;
mod stage;
mod state;

use bevy::prelude::*;
use bevy_rng::RngPlugin;

use crate::arch::{DEFAULT_PLUGINS, RUNNER};
use crate::tracing;

use label::SystemLabel;
use plugin::AppPlugin;
use stage::AppStage;
pub(crate) use state::AppState;

// Temporary hack until `on_state_enter` works as expected.
//
// See: <https://github.com/bevyengine/bevy/issues/1117>
#[derive(Default, Debug)]
pub(crate) struct OnStateEnterFix {
    pub tileset_spawn: bool,
    pub player_spawn: bool,
    pub camera_spawn: bool,
}

pub struct Game;

impl Game {
    #[tracing::instrument(level = "warn")]
    pub fn run() {
        App::build()
            // Debug
            .add_plugin(plugin::DebugPlugin)
            // General
            .add_plugins(DEFAULT_PLUGINS)
            .insert_resource(RUNNER)
            // Window
            .insert_resource(WindowDescriptor {
                title: "Exodus: The Morning Star".to_string(),
                width: 1080.,
                height: 675.,
                vsync: false,
                resizable: true,
                ..Default::default()
            })
            .insert_resource(ClearColor(Color::BLACK))
            // Stages
            .add_stage_before(
                CoreStage::Update,
                AppStage::Update,
                StateStage::<AppState>::default(),
            )
            .add_stage_after(
                AppStage::Update,
                AppStage::Physics,
                StateStage::<AppState>::default(),
            )
            .add_stage_after(
                AppStage::Physics,
                AppStage::View,
                StateStage::<AppState>::default(),
            )
            // App State
            .insert_resource(State::new(AppState::Loading))
            // Plugins
            .add_plugin(RngPlugin::from("<random seed>"))
            .add_plugin(AppPlugin)
            .run();
    }
}
