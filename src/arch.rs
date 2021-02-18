use bevy::app::{RunMode, ScheduleRunnerSettings};

pub(super) const RUNNER: ScheduleRunnerSettings = if cfg!(feature = "headless") {
    ScheduleRunnerSettings {
        run_mode: RunMode::Once,
    }
} else {
    ScheduleRunnerSettings {
        run_mode: RunMode::Loop { wait: None },
    }
};

cfg_if::cfg_if! {
    if #[cfg(feature = "headless")] {
        use bevy::MinimalPlugins;
        pub(super) const PLUGINS: MinimalPlugins = MinimalPlugins;
    } else if #[cfg(target_arch = "wasm32")]  {
        use bevy_webgl2::DefaultPlugins;
        pub(super) const PLUGINS: DefaultPlugins = DefaultPlugins;
    } else {
        use bevy::DefaultPlugins;
        pub(super) const PLUGINS: DefaultPlugins = DefaultPlugins;
    }
}
