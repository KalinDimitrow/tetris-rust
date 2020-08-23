extern crate find_folder;
use piston_window::*;
use std::error;

const ASSET_DIRECTORY : &str = "assets";
const GAME_FONT : &str = "TetrisFont2.ttf";
const PARENT_DEPTH : find_folder::ParentsDepth = 3;
const KIDS_DEPTH : find_folder::KidsDepth = 3;

pub struct GameResources {
    pub background : G2dTexture,
    pub empty_block : G2dTexture,
    pub cube_block : G2dTexture,
    pub font : Glyphs,
}

impl GameResources {
    pub fn new(_path : &str,  window : &mut PistonWindow) -> Result<GameResources, Box<dyn error::Error>> {

        let assets = find_folder::Search::ParentsThenKids(PARENT_DEPTH, KIDS_DEPTH)
            .for_folder(ASSET_DIRECTORY)?;

        let _tex = window.create_texture_context();

        let background = Texture::from_path(
            &mut window.create_texture_context(),
            assets.join(String::from("background2.png")),
            Flip::None,
            &TextureSettings::new()
        )?;

        let empty_block = Texture::from_path(
            &mut window.create_texture_context(),
            assets.join(String::from("empty_block2.png")),
            Flip::None,
            &TextureSettings::new()
        )?;

        let cube_block = Texture::from_path(
            &mut window.create_texture_context(),
            assets.join(String::from("cube_block2.png")),
            Flip::None,
            &TextureSettings::new()
        )?;

        let ref font = assets.join(GAME_FONT);
        let font = window.load_font(font)?;

        let result = GameResources {
            background : background,
            empty_block : empty_block,
            cube_block : cube_block,
            font : font
        };

        Ok(result)
    }
}
