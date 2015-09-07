//! # Render Window
//!
//! The render window is used to create an environment to easily draw objects two, and get events
//! from.
// TODO: Would it make a notable difference to do all the rendering in a different thread? All
// drawables would have to be shareable.

use sdl2::{EventPump, Sdl};
use sdl2::render::Renderer;
pub use sdl2::event::EventPollIterator;

use super::texture_manager::TextureManager;
use super::sprite::Sprite;
use super::drawable::Drawable;
use std::sync::{Arc, Mutex};
use time::{self, Timespec};

pub struct RenderWindow<'a> {
	context: &'a Sdl,
	event_pump: EventPump,

	renderer: Arc<Mutex<Renderer<'static>>>,
	textures: TextureManager,

	last_time: Timespec,
	frame_dur: f32
}

impl<'a> RenderWindow<'a> {
	pub fn new(context: &'a Sdl, title: &str, width: u32, height: u32) -> RenderWindow<'a> {
		let video_subsystem = context.video().unwrap();
		let event_pump = context.event_pump().unwrap();

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
			event_pump: event_pump,
			renderer: renderer,
			textures: textures,

			last_time: time::get_time(),
			frame_dur: 0.1
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
		// Measure the framerate
		let cur_time = time::get_time();
		let frame_dur = cur_time - self.last_time;
		self.last_time = cur_time;

		// convert the exact duration to the approximate f32 value.
		self.frame_dur = frame_dur.num_microseconds().unwrap() as f32 / 1000000.0;

		let mut renderer = self.renderer.lock().unwrap();
		renderer.present();
	}

	/// The framerate
	///
	/// Returns the current framerate of the window in frames/sec.
	pub fn fps(&self) -> u16 {
		(1.0 / self.frame_dur) as u16
	}

	/// # The frame duration
	///
	/// Returns the duration of the last measured frame in seconds.
	pub fn frame_duration(&self) -> f32 {
		self.frame_dur
	}

	/// # Get the event iterator
	///
	/// Polls all the events that have not been processed before from the event pump.
	pub fn poll_events(&mut self) -> EventPollIterator {
		self.event_pump.poll_iter()
	}
}
