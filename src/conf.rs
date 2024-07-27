//! Context creation configuration
//!
//! A [`Conf`] struct is used to describe a hardware and platform specific setup,
//! mostly video display settings.
//!
//! ## High DPI rendering
//!
//! You can set the [`Conf::high_dpi`](Conf::high_dpi) flag during initialization to request
//! a full-resolution framebuffer on HighDPI displays. The default behaviour
//! is `high_dpi = false`, this means that the application will
//! render to a lower-resolution framebuffer on HighDPI displays and the
//! rendered content will be upscaled by the window system composer.
//! In a HighDPI scenario, you still request the same window size during
//! [`miniquad_wasm_bindgen::start`](crate::start), but the framebuffer sizes returned by [`window::screen_size`](crate::window::screen_size)
//! will be scaled up according to the DPI scaling ratio.
//! You can also get a DPI scaling factor with the function
//! [`window::dpi_scale`](crate::window::screen_size).
//! Here's an example on a Mac with Retina display:
//!
//! ```ignore
//! Conf {
//!   width = 640,
//!   height = 480,
//!   high_dpi = true,
//!   .. Default::default()
//! };
//! ```
//!
//! The functions [`window::screen_size`](crate::window::screen_size) and [`window::dpi_scale`](crate::window::screen_size) will
//! return the following values:
//! ```bash
//! screen_size -> (1280, 960)
//! dpi_scale   -> 2.0
//! ```
//!
//! If the high_dpi flag is false, or you're not running on a Retina display,
//! the values would be:
//! ```bash
//! screen_size -> (640, 480)
//! dpi_scale   -> 1.0
//! ```

#[derive(Debug, Clone)]
pub enum LinuxX11Gl {
	/// Use libGLX.so/libGLX.so.0 and its funciton for creating OpenGL context
	/// If there is no libGLX - just panic right away
	GLXOnly,
	/// Use libEGL.so/libEGL.so.0 and its funciton for creating OpenGL context
	/// If there is no libEGL - just panic right away
	EGLOnly,
	/// Use libGLX and if there is not libGLX - try libEGL.
	/// The default option.
	GLXWithEGLFallback,
	/// Use libEGL and if there is not libEGL - try libGLX.
	EGLWithGLXFallback,
}

#[derive(Debug, Clone)]
pub enum LinuxBackend {
	X11Only,
	WaylandOnly,
	X11WithWaylandFallback,
	WaylandWithX11Fallback,
}

/// Platform specific settings.
#[derive(Debug, Clone)]
pub struct PlatformSettings {
	/// On X11 there are two ways to get OpenGl context: libglx.so and libegl.so
	/// Default is GLXWithEGLFallback - will try to create glx context and if fails -
	/// try EGL. If EGL also fails - panic.
	pub linux_x11_gl: LinuxX11Gl,

	/// Wayland or X11. Defaults to X11WithWaylandFallback - miniquad_wasm_bindgen will try
	/// to load "libX11.so", but if there is no - will try to initialize
	/// through wayland natively. If both  fails (no graphics server at
	/// all, like KMS) - will panic.
	///
	/// Defaults to X11Only. Wayland implementation is way too unstable right now.
	pub linux_backend: LinuxBackend,

	/// On some platform it is possible to ask the OS for a specific swap interval.
	/// Note that this is highly platform and implementation dependent,
	/// there is no guarantee that FPS will be equal to swap_interval.
	/// In other words - "swap_interval" is a hint for a GPU driver, this is not
	/// the way to limit FPS in the game!
	pub swap_interval: Option<i32>,

	/// A way to reduce CPU usage to zero when waiting for an incoming event.
	/// update()/draw() will only be called after `window::request_update()`.
	/// It is recommended to put `request_update` at the end of `resize_event` and
	/// relevant mouse/keyboard input.
	/// `request_update` may be used from other threads to "wake up" the window.
	pub blocking_event_loop: bool,

	/// On Web, sets `WebGlContextAttributes.alpha = true`.
	/// On Linux, enables transparent windows.
	pub framebuffer_alpha: bool,

	/// Whether to draw the default window decorations on Wayland.
	/// Only works when using the Wayland backend.
	pub wayland_use_fallback_decorations: bool,

	/// Html Query Selector for the canvas element to attach to.
	pub web_canvas_query_selector: &'static str,
}

impl Default for PlatformSettings {
	fn default() -> PlatformSettings {
		PlatformSettings {
			linux_x11_gl: LinuxX11Gl::GLXWithEGLFallback,
			swap_interval: None,
			blocking_event_loop: false,
			// TODO: Wayland Backend is very incomplete
			linux_backend: LinuxBackend::X11Only,
			framebuffer_alpha: false,
			wayland_use_fallback_decorations: true,
			web_canvas_query_selector: "#glcanvas",
		}
	}
}

#[derive(Debug)]
pub struct Conf {
	/// Title of the window, defaults to an empty string.
	pub window_title: String,
	/// The preferred width of the window or canvas.
	pub window_width: u32,
	/// The preferred height of the window or canvas.
	pub window_height: u32,
	/// Whether the rendering canvas is full-resolution on HighDPI displays.
	pub high_dpi: bool,
	/// Whether the window should be created in fullscreen mode, ignored on web.
	pub fullscreen: bool,
	/// MSAA sample count
	pub sample_count: i32,

	/// Determines if the application user can resize the window
	pub window_resizable: bool,

	/// Miniquad allows to change the window icon programmatically.
	/// The icon will be used as
	/// - taskbar and titlebar icons on Windows.
	/// - TODO: favicon on HTML5
	/// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
	pub icon: Option<Icon>,

	/// Platform specific settings. Hints to OS for context creation, driver-specific
	/// settings etc.
	pub platform: PlatformSettings,
}

/// Icon image in three levels of detail.
#[derive(Clone)]
pub struct Icon {
	/// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
	pub small: [u8; 16 * 16 * 4],
	/// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
	pub medium: [u8; 32 * 32 * 4],
	/// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
	pub big: [u8; 64 * 64 * 4],
}

impl Icon {
	pub fn miniquad_logo() -> Icon {
		Icon {
			small: crate::default_icon::SMALL,
			medium: crate::default_icon::MEDIUM,
			big: crate::default_icon::BIG,
		}
	}
}
// Printing 64x64 array with a default formatter is not meaningfull,
// so debug will skip the data fields of an Icon
impl std::fmt::Debug for Icon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Icon").finish()
	}
}

// reasonable defaults for PC and mobiles are slightly different
impl Default for Conf {
	fn default() -> Conf {
		Conf {
			window_title: "Miniquad WBG Window".to_owned(),
			window_width: 800,
			window_height: 600,
			high_dpi: false,
			fullscreen: false,
			sample_count: 1,
			window_resizable: true,
			icon: Some(Icon::miniquad_logo()),
			platform: Default::default(),
		}
	}
}
