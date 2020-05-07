use std::collections::HashMap;
use std::sync::{Arc};
use std::env;
use sfml::system::SfBox;
use sfml::graphics::{Texture, Font};

pub struct ResourceManager {
    textures: HashMap<String, Arc<SfBox<Texture>>>,
    fonts: HashMap<String, Arc<SfBox<Font>>>,
}

fn resolve_asset_path(filename: &'static str) -> Option<String> {
    if let Ok(mut absolute_path) = env::current_dir() {
        absolute_path.push("assets/");
        absolute_path.push(filename);
        return Some(absolute_path.into_os_string().into_string().unwrap());
    }
    None
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            textures: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn get_texture(&mut self, filename: &'static str) -> Arc<SfBox<Texture>> {
        // could also do the following:
        // let ffp = resolve_asset_path
        // let arc = etc
        // if ffp.is_empty || arc.is_empty then return default texture
        let full_file_path = resolve_asset_path(filename).unwrap();
        let arc = self.textures.entry(full_file_path.clone()).or_insert(
            match Texture::from_file(full_file_path.as_str()) {
                Some(tex) => Arc::new(tex),
                None => panic!("Load a default texture u spaztique")
            }
        );
        return arc.clone();
    }

    pub fn get_font(&mut self, filename: &'static str) -> Arc<SfBox<Font>> {
        let full_file_path = resolve_asset_path(filename).unwrap();
        let arc = self.fonts.entry(filename.to_owned()).or_insert(
            match Font::from_file(filename) {
                Some(font) => Arc::new(font),
                None => panic!("Load a default font u spaztique")
            }
        );
        return arc.clone();
    }

    pub fn unload(&mut self) {
        self.textures.retain(|_, arc| Arc::strong_count(&arc) > 1);
        self.fonts.retain(|_, arc| Arc::strong_count(&arc) > 1);
    }

}