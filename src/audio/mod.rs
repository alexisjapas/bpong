use bevy::prelude::*;

pub struct AudioPlugin;

#[derive(Resource)]
pub struct SoundAssets {
    pub ping: Handle<AudioSource>,
    pub pong: Handle<AudioSource>,
    pub ee: Vec<Handle<AudioSource>>,
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sounds);
    }
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        ping: asset_server.load("audio/ping.ogg"),
        pong: asset_server.load("audio/pong.ogg"),
        ee: (1..=7)
            .map(|i| asset_server.load(format!("audio/ee/pong_{:02}.ogg", i)))
            .collect(),
    });
}
