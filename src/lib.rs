use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::IoTaskPool;
use bevy_tasks::futures_lite::StreamExt;
use mundy::Interest;
pub use mundy::{ColorScheme, Contrast, DoubleClickInterval, ReducedMotion, ReducedTransparency};

mod preferences;
pub use preferences::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct MundySystems;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct MundyPlugin {}

impl Plugin for MundyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Preferences>()
            .configure_sets(Startup, MundySystems)
            .configure_sets(PreUpdate, MundySystems)
            .init_resource::<Preferences>()
            .add_systems(Startup, subscribe_to_preferencs.in_set(MundySystems))
            .add_systems(PreUpdate, poll_system_preferences.in_set(MundySystems));
    }
}

fn subscribe_to_preferencs(mut commands: Commands) {
    let (tx, rx) = crossbeam_channel::unbounded();
    let stream = mundy::Preferences::stream(Interest::All);
    IoTaskPool::get()
        .spawn(async move { forward_stream_to_receiver(tx, stream).await })
        .detach();
    commands.insert_resource(Receiver(rx));
}

async fn forward_stream_to_receiver(
    sender: crossbeam_channel::Sender<mundy::Preferences>,
    mut stream: mundy::PreferencesStream,
) {
    while let Some(preferences) = stream.next().await {
        _ = sender.send(preferences);
    }
}

#[derive(Debug, Resource)]
struct Receiver(crossbeam_channel::Receiver<mundy::Preferences>);

fn poll_system_preferences(
    receiver: Res<Receiver>,
    mut preferences_res: ResMut<Preferences>,
) -> Result {
    let preferences = match receiver.0.try_recv() {
        Ok(preferences) => preferences,
        Err(crossbeam_channel::TryRecvError::Empty) => return Ok(()),
        Err(e) => return Err(e.into()),
    };
    *preferences_res = preferences.into();
    Ok(())
}
