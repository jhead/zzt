use bevy::{prelude::*, sprite::Material2dPlugin};
use zzt::{
    binary::{BinaryAsset, BinaryAssetPlugin},
    char::CharacterMaterial,
    font::DosFont,
};
use zzt::{char::Char, state::State};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(Material2dPlugin::<CharacterMaterial>::default())
        .add_plugin(BinaryAssetPlugin)
        .init_resource::<State>()
        .add_startup_system(startup)
        .add_system(system_build_fonts)
        .add_system(system_build_tiles)
        .run();
}

fn startup(mut state: ResMut<State>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    state.font_handle = asset_server.load("CP437.F08");
}

fn system_build_fonts(
    files: ResMut<Assets<BinaryAsset>>,
    images: ResMut<Assets<Image>>,
    mut state: ResMut<State>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if state.font_loaded {
        return;
    }

    let font_asset = files.get(&state.font_handle);
    if let Some(font_asset) = font_asset {
        // TODO: is clone necessary?
        let font = DosFont::new_8bit(font_asset.to_owned());

        let atlas = font.build_texture_atlas(images);
        state.char_atlas = texture_atlases.add(atlas);
        state.font_loaded = true;
    }
}

fn system_build_tiles(
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State>,
    mut commands: Commands,
    mut materials: ResMut<Assets<CharacterMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if state.tiles_added || !state.font_loaded {
        return;
    }

    let map_size: [u32; 2] = [40, 25];

    for x in 0..map_size[0] {
        for y in 0..map_size[1] {
            let tile_pos = Vec2 {
                x: (x * 8) as f32,
                y: (y * 8) as f32,
            };

            let char = Char {
                code: (((y * 40) + x) % 255) as u8,
                color_fg: Color::WHITE,
                color_bg: Color::NONE,
                transform: Transform::from_xyz(tile_pos.x, tile_pos.y, 0.),
            };

            let char_bundle =
                char.create_bundle(&state, &texture_atlases, &mut materials, &mut meshes);
            commands.spawn(char_bundle);
        }
    }

    state.tiles_added = true;
}
