/*
 * The texture manager coordinates the allocation of textures, especially those, that are loaded
 * from SDL Surfaces and May be used more than just one time.
 */

extern crate sdl2;
use sdl2::render::{Texture, Renderer};
use sdl2::surface::Surface;
use sdl2::{SdlResult};

use std::collections::HashMap;
use std::path::Path;

/// Internal structure, used to save the Texture itself, and some other data that may be important,
/// like the original size of the Surface, which could otherwise not be accessed any longer.
pub struct TextureEntry {
    texture: Texture,
    width: u32,
    height: u32
}

pub struct TextureManager<'a> {
    renderer: &'a Renderer<'a>,
    textures: HashMap<String, TextureEntry>
}

impl TextureEntry {
    pub fn new(texture: Texture, width: u32, height: u32) -> TextureEntry {
        TextureEntry {
            texture: texture,
            width: width,
            height: height
        }
    }

    pub fn from_surface(renderer: &Renderer, surface: Surface) -> SdlResult<TextureEntry> {
        Ok(TextureEntry {
            width: surface.width(),
            height: surface.height(),
            texture: match renderer.create_texture_from_surface(surface) {
                Ok(surface) => surface,
                Err(err) => return Err(err)
            }
        })
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<'a> TextureManager<'a> {
    pub fn new(renderer: &'a Renderer) -> TextureManager<'a> {
        TextureManager {
            renderer: renderer,
            textures: HashMap::new()
        }
    }

    /// # Search a texture by name.
    ///
    /// This will look up the texture in the map, and if it cannot find it, it will additionally
    /// look for a file corresponding to this name. Upon finding it, it will then return a
    /// reference to this texture.
    ///
    /// # Error
    /// In case the file cannot be found, the function will return 'None'.
    pub fn load_texture(&mut self, name: &str) -> Option<&TextureEntry> {
        // This check is necessary, to convince the borrow-checker everything is in order.
        // TODO: Maybe there is a better solution, without checking the value twice.
        if self.textures.contains_key(&String::from(name)) {
            // The texture had already been loaded.
            self.textures.get(&String::from(name))
        }
        else {
            // The texture had not yet been loaded.
            let surface = match Surface::load_bmp(Path::new(name)) {
                Ok(surface) => surface,
                Err(err) => {
                    println!("Error occured loading {}. {}", name, err);
                    return None;
                }
            };

            let texture_entry = match TextureEntry::from_surface(self.renderer, surface) {
                Ok(texture_entry) => texture_entry,
                Err(err) => {
                    println!("Error occured creating texture for {}. {}", name, err);
                    return None;
                }
            };

            // After this it is certain, that the texture entry has been loaded correctly, now it has
            // to be saved in the map to avoid duplication.
            self.textures.insert(String::from(name), texture_entry);
            self.textures.get(&String::from(name))
        }
    }

    /// # Search a texture by name.
    ///
    /// Unlike load_texture, this function only looks the texture up in the map of textures, that
    /// are already loaded. This makes a call to this function much faster.
    ///
    /// # Error
    /// In case the texture cannot be found, the program panic!s. This behaviour is appropriate,
    /// since otherwise uncomfortable programming errors could be ignored.
    pub fn get_texture(&self, name: &str) -> &TextureEntry {
        self.textures.get(&String::from(name)).expect(format!("The texture {} should have already been loaded, but could not be found.", name).as_str())
    }
}
