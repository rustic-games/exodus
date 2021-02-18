use bevy::app::PluginGroupBuilder;
use bevy::core::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

use crate::arch::{PLUGINS, RUNNER};
use crate::event::Move;
use crate::state::{Camera, World};
use crate::system::*;

pub struct Game;

impl Game {
    pub fn run() {
        let window_background = ClearColor(Color::rgb(0., 0., 0.));
        let mssa = Msaa { samples: 4 };
        let logger = LogSettings {
            level: Level::INFO,
            filter: "wgpu=warn,bevy_ecs=info".to_string(),
        };

        App::build()
            .insert_resource(WindowDescriptor {
                title: "Exodus: The Morning Star".to_string(),
                width: 1440.,
                height: 900.,
                vsync: true,
                ..Default::default()
            })
            .add_event::<Move>()
            .add_startup_stage("game_boot", SystemStage::single(setup.system()))
            .add_startup_stage(
                "game_setup",
                SystemStage::parallel()
                    .with_system(spawn_world.system())
                    .with_system(spawn_player.system()),
            )
            .add_stage_after(
                stage::UPDATE,
                "fixed_update",
                SystemStage::parallel().with_run_criteria(FixedTimestep::steps_per_second(100.0)),
            )
            .add_system(move_player.system())
            .add_system(move_entity.system())
            .add_system(exit_on_esc_system.system())
            .insert_resource(mssa)
            .insert_resource(logger)
            .insert_resource(RUNNER)
            .insert_resource(window_background)
            .init_resource::<World>()
            .init_resource::<Camera>()
            .add_plugins(PLUGINS)
            .add_plugin(bevy_rng::RngPlugin::from("foobar"))
            .add_plugins(DiagnosticsPlugins)
            .run()
    }
}

struct DiagnosticsPlugins;

impl PluginGroup for DiagnosticsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(LogDiagnosticsPlugin::default())
            .add(FrameTimeDiagnosticsPlugin::default());
    }
}
