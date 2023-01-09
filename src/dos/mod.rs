use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

use font::DosFont;

use crate::binary::{BinaryAsset, BinaryAssetPlugin};
pub use crate::dos::char::*;

pub mod char;
mod font;

pub struct DosPlugin;

impl Plugin for DosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Terminal>()
            .init_resource::<PluginState>()
            .add_plugin(BinaryAssetPlugin)
            .add_plugin(Material2dPlugin::<TerminalCharMaterial>::default())
            .add_system(system_update_chars)
            .add_system(system_spawn_chars)
            .add_system(system_build_fonts)
            .add_startup_system(startup_load_fonts);
    }
}

#[derive(Resource, Default)]
struct PluginState {
    font_handle: Handle<BinaryAsset>,
}

#[derive(Resource)]
pub struct Terminal {
    pub size: UVec2,
    pub char_size: UVec2,
    char_queue: Vec<(Option<Entity>, TerminalChar)>,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            size: UVec2 { x: 40, y: 25 },
            char_size: UVec2 { x: 8, y: 8 },
            char_queue: default(),
        }
    }
}

impl Terminal {
    pub fn base_resolution(&self) -> UVec2 {
        UVec2 {
            x: self.size.x * self.char_size.x,
            y: self.size.y * self.char_size.y,
        }
    }

    pub fn camera_translation(&self) -> Vec2 {
        let res = self.base_resolution();

        Vec2 {
            x: res.x as f32 / 2.,
            y: res.y as f32 / 2.,
        }
    }

    pub fn loc_to_world(&self, loc: UVec3) -> Vec3 {
        let res = self.base_resolution();
        let x = (loc.x * self.char_size.x) + (self.char_size.x / 2);
        let y = res.y - (loc.y * self.char_size.y) - (self.char_size.y / 2);
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: loc.z as f32,
        }
    }

    pub fn add_char(&mut self, char: TerminalChar, parent: Option<Entity>) {
        self.char_queue.push((parent, char));
    }
}

fn system_spawn_chars(
    atlas: Option<Res<TerminalTextureAtlas>>,
    mut commands: Commands,
    mut term: ResMut<Terminal>,
    mut materials: ResMut<Assets<TerminalCharMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Some(atlas) = atlas {
        for (parent, char) in term.char_queue.drain(..) {
            println!("Spawning char {} {}", char.code, char.loc);
            let bundle = create_bundle(char, &atlas.0, &mut materials, &mut meshes);
            let entity = commands.spawn(bundle).id();

            if let Some(parent) = parent {
                commands.entity(parent).push_children(&[entity]);
            }
        }
    }
}

fn system_update_chars(
    mut term: ResMut<Terminal>,
    mut chars: Query<(&TerminalChar, &mut Transform)>,
) {
    for (char, mut transform) in chars.iter_mut() {
        // FIXME: really inefficient
        let new_translation = term.loc_to_world(char.loc);
        transform.translation = new_translation;
        transform.scale.x = term.char_size.x as f32;
        transform.scale.y = term.char_size.y as f32;
    }
}

fn startup_load_fonts(
    mut state: ResMut<PluginState>,
    asset_server: Res<AssetServer>,
) {
    println!("Loading font");
    state.font_handle = asset_server.load("CP437.F08");
}

fn system_build_fonts(
    atlas: Option<Res<TerminalTextureAtlas>>,
    files: Res<Assets<BinaryAsset>>,
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    state: ResMut<PluginState>,
) {
    if atlas.is_some() {
        return;
    }

    let font_asset = files.get(&state.font_handle);
    if let Some(font_asset) = font_asset {
        println!("Building font");
        // TODO: is clone necessary?
        let font = DosFont::new_8bit(font_asset.to_owned());

        let atlas = font.build_texture_atlas(images);
        commands.insert_resource(TerminalTextureAtlas(atlas));
    }
}
