use bevy::prelude::*;
use image::{DynamicImage, GrayAlphaImage, LumaA};

use crate::binary::BinaryAsset;

pub struct DosFont {
    pub image: BinaryAsset,
    pub num_columns: u32,
    pub num_rows: u32,
    pub char_width: u32,
    pub char_height: u32,
    pub bytes_per_char: u32,
}

impl DosFont {
    pub fn new_8bit(image: BinaryAsset) -> DosFont {
        DosFont {
            image: image,
            num_columns: 16,
            num_rows: 16,
            char_width: 8,
            char_height: 8,
            bytes_per_char: 8,
        }
    }

    pub fn into_image(self) -> Image {
        self.into()
    }

    pub fn build_texture_atlas(self, mut images: ResMut<Assets<Image>>) -> TextureAtlas {
        let tile_size = Vec2::new(self.char_width as f32, self.char_height as f32);
        let num_columns = self.num_columns;
        let num_rows = self.num_rows;

        let texture = self.into_image();
        let texture_handle = images.add(texture);

        TextureAtlas::from_grid(
            texture_handle,
            tile_size,
            num_columns as usize,
            num_rows as usize,
            None,
            None,
        )
    }
}

impl Into<Image> for DosFont {
    fn into(self) -> Image {
        let map_width = self.num_columns * self.char_width;
        let map_height = self.num_rows * self.char_height;

        let img = GrayAlphaImage::from_fn(map_width, map_height, |x, y| {
            let atlas_sprite_index =
                (x / self.char_width) + ((y / self.char_height) * self.num_columns);
            let atlas_row = atlas_sprite_index / self.num_columns;

            // 8 bits for 8 x pixels, 1 byte per y, 8 bytes per sprite
            let y_offset = y - (atlas_row * self.char_height);
            let byte_pos_in_file = ((atlas_sprite_index * self.bytes_per_char) + y_offset) as usize;

            // Extract boolean luma from bit flag baesd on x position in sprite
            let luma_binary_mask = self.image.data[byte_pos_in_file] >> (x % 8);

            // On bit == white, off bit == black
            let luma_binary = (luma_binary_mask & 1) == 1;

            // Convert binary to luma; swap black for transparent alpha
            let luma: u8 = if luma_binary { 255 } else { 0 };
            let alpha = luma;

            LumaA::from([luma, alpha])
        });

        Image::from_dynamic(DynamicImage::ImageLumaA8(img), false)
    }
}
