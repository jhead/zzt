use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct TerminalChar {
    pub code: u8,
    pub loc: UVec3,
    pub background: Color,
    pub foreground: Color,
}

#[derive(Resource)]
pub struct TerminalTextureAtlas(pub TextureAtlas);

#[derive(Bundle)]
pub struct CharBundle {
    char: TerminalChar,
    mesh: MaterialMesh2dBundle<TerminalCharMaterial>,
}

pub fn create_bundle(
    char: TerminalChar,
    atlas: &TextureAtlas,
    materials: &mut ResMut<Assets<TerminalCharMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> CharBundle {
    let atlas_texture = atlas.texture.clone();
    let sprite_rect = atlas.textures[char.code as usize];

    let atlas_size = Vec2 { x: 128., y: 128. }; // FIXME

    // Build a default quad mesh
    let mut mesh = Mesh::from(shape::Quad::default());

    // Sprite rect normalized against atlas texture size
    let sprite_min_norm = sprite_rect.min / atlas_size;
    let sprite_max_norm = sprite_rect.max / atlas_size;
    let uvs = vec![
        [sprite_min_norm.x, sprite_max_norm.y],
        [sprite_min_norm.x, sprite_min_norm.y],
        [sprite_max_norm.x, sprite_min_norm.y],
        [sprite_max_norm.x, sprite_max_norm.y],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    // Spawn the quad with vertex colors
    let mesh_bundle = MaterialMesh2dBundle {
        mesh: mesh_handle.clone(),
        material: materials.add(TerminalCharMaterial {
            texture: Some(atlas_texture),
            foreground_color: char.foreground,
            background_color: char.background,
        }),
        ..default()
    };

    CharBundle {
        char,
        mesh: mesh_bundle,
    }
}

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[reflect(Default, Debug)]
#[uuid = "f1720c79-4b4b-4284-818b-2054ac17549c"]
pub struct TerminalCharMaterial {
    #[uniform(0)]
    pub foreground_color: Color,
    #[uniform(1)]
    pub background_color: Color,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
}

impl Default for TerminalCharMaterial {
    fn default() -> Self {
        TerminalCharMaterial {
            foreground_color: Color::GREEN,
            background_color: Color::BLUE,
            texture: None,
        }
    }
}

impl Material2d for TerminalCharMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/character_mat.wgsl".into()
    }
}
