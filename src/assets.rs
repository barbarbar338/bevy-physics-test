use bevy::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub duck: Handle<Scene>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        font: assets.load("fonts/hubot-sans.ttf"),
        duck: assets.load("models/duck.gltf#Scene0"),
    });
}
