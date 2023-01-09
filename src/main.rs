use bevy::{prelude::*};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

use zzt::dos::{DosPlugin, Terminal, TerminalChar};
use zzt::state::State;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::BLACK,
        })
        .add_plugin(DosPlugin)
        .init_resource::<State>()
        .add_startup_system(startup)
        .add_system(system_build_tiles)
        .run();
}

fn startup(mut commands: Commands, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_resolution(320. * 3., 200. * 3.);
    }

    commands.spawn(PixelCameraBundle::from_resolution(320, 200));
}

fn system_build_tiles(
    mut term: ResMut<Terminal>,
    mut state: ResMut<State>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if state.tiles_added {
        return;
    }

    let mut camera = camera_query.single_mut();
    let camera_translation = term.camera_translation();
    camera.translation.x = camera_translation.x;
    camera.translation.y = camera_translation.y;

    for x in 0..term.size.x {
        for y in 0..term.size.y {
            let code = (((y * term.size.x) + x) % 255) as u8;
            let char = TerminalChar {
                code,
                loc: UVec3 { x, y, z: 0 },
                background: Color::NONE,
                foreground: Color::WHITE,
            };

            term.add_char(char, None);
        }
    }

    state.tiles_added = true;
}
