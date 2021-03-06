/*
 * Abstract graphical elements are part of this module. This does not include actual in-game usage.
 * It is only supposed to create another layer of abstraction, more specific than the one already
 * provided by SDL.
 */

pub mod drawable;
pub mod render_window;
pub mod sprite;
pub mod texture_manager;

pub use self::drawable::Drawable;
pub use self::render_window::RenderWindow;
pub use self::sprite::Sprite;
pub use self::texture_manager::TextureManager;
