//! # Render Window
//!
//! The render window is used to create an environment to easily draw objects two, and get events
//! from.
// TODO: Would it make a notable difference to do all the rendering in a different thread? All
// drawables would have to be shareable.

use sdl2::Sdl;
use sdl2::render::Renderer;
use super::texture_manager::TextureManager;
use super::sprite::Sprite;
use super::drawable::Drawable;
use std::sync::{Arc, Mutex};

pub struct RenderWindow<'a> {
	context: &'a Sdl,

	renderer: Arc<Mutex<Renderer<'static>>>,
	textures: TextureManager
}

impl<'a> RenderWindow<'a> {
	pub fn new(context: &'a Sdl, title: &str, width: u32, height: u32) -> RenderWindow<'a> {
		let video_subsystem = context.video().unwrap();

		// Using some very common settings here. Sadly, Fullscreen in Linux is broken, but to be
		// honest, I always thought it should be handled by the window manager anyhow.
		let window = video_subsystem.window(title, width, height).position_centered().opengl().build().unwrap();

		// Build the renderer that is used to draw anything to the window. Currently it is always
		// hardware accelerated.
		let renderer = Arc::new(Mutex::new(window.renderer().accelerated().build().unwrap()));

		// The texture manager, dependent on the renderer, since all textures are bound to the
		// graphics card on which the renderer is running.
		let textures = TextureManager::new(renderer.clone());

		// At this point everything is initialised.
		RenderWindow {
			context: context,
			renderer: renderer,
			textures: textures
		}
	}

	/// # Create a sprite
	///
	/// This function creates a new sprite bound to the renderer of this window. It takes the name
	/// of the texture.
	///
	/// # Failure
	/// In case the texture cannot be loaded, this function returns an error code.
	pub fn create_sprite(&mut self, texture: &str, pos: (f32, f32), size: Option<(u32, u32)>) -> Result<Sprite, String> {
		// TODO: get_texture() is significantly faster. There must be a way to determine when to
		// load something to not use this all the time, and not load everything simply at program
		// launch. Also there would be no need for the window to be mutable at all times.
		let texture_entry = match self.textures.load_texture(texture) {
			Some(entry) => entry,
			None => return Err(format!("Texture {} could not be found.", texture))
		};

		Ok(Sprite::new(&texture_entry, pos, size))
	}

	/// # Draw something
	///
	/// Draw the provided object to this window.
	pub fn draw<T: Drawable>(&mut self, obj: &T) {
		let mut renderer = self.renderer.lock().unwrap();
		obj.draw(&mut renderer);
	}

	/// # Clear the screen
	///
	/// Clear the screen in all black colour to avoid artifacts that otherwise may occur.
	pub fn clear(&mut self) {
		let mut renderer = self.renderer.lock().unwrap();
		renderer.clear();
	}

	/// # Present the screen
	///
	/// Flips the current drawing buffer to present the newly created frame.
	pub fn present(&mut self) {
		let mut renderer = self.renderer.lock().unwrap();
		renderer.present();
	}
}
