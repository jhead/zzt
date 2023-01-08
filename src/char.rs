use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::state::State;

#[derive(Component)]
pub struct Char {
    pub code: u8,
    pub color_fg: Color,
    pub color_bg: Color,
    pub transform: Transform,
}

#[derive(Bundle)]
pub struct CharBundle {
    char: Char,
    mesh: MaterialMesh2dBundle<CharacterMaterial>,
}

impl Char {
    pub fn create_bundle(
        self,
        state: &ResMut<State>,
        texture_atlases: &ResMut<Assets<TextureAtlas>>,
        materials: &mut ResMut<Assets<CharacterMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> CharBundle {
        let atlas = texture_atlases.get(&state.char_atlas.to_owned()).unwrap();
        let atlas_texture = atlas.texture.clone();
        let sprite_rect = atlas.textures[self.code as usize];

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
            transform: self.transform.with_scale(Vec3::splat(8.)),
            material: materials.add(CharacterMaterial {
                texture: Some(atlas_texture),
                foreground_color: self.color_fg,
                background_color: self.color_bg,
            }),
            ..default()
        };

        CharBundle {
            char: self,
            mesh: mesh_bundle,
        }
    }
}

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[reflect(Default, Debug)]
#[uuid = "f1720c79-4b4b-4284-818b-2054ac17549c"]
pub struct CharacterMaterial {
    #[uniform(0)]
    pub foreground_color: Color,
    #[uniform(1)]
    pub background_color: Color,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
}

impl Default for CharacterMaterial {
    fn default() -> Self {
        CharacterMaterial {
            foreground_color: Color::GREEN,
            background_color: Color::BLUE,
            texture: None,
        }
    }
}

impl Material2d for CharacterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/character_mat.wgsl".into()
    }
}
