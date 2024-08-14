#![doc = include_str!("../README.md")]
#![allow(clippy::too_many_arguments)]

pub mod conf;
mod event;
pub mod fs;
pub mod graphics;
mod native;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[cfg(feature = "log-impl")]
pub mod log;

pub use event::*;

pub use graphics::*;

mod default_icon;

pub use native::gl;

#[derive(Clone)]
pub(crate) struct ResourceManager<T> {
	id: usize,
	resources: HashMap<usize, T>,
}

impl<T> Default for ResourceManager<T> {
	fn default() -> Self {
		Self { id: 0, resources: HashMap::new() }
	}
}

impl<T> ResourceManager<T> {
	pub fn add(&mut self, resource: T) -> usize {
		self.resources.insert(self.id, resource);
		self.id += 1;
		self.id - 1
	}

	pub fn remove(&mut self, id: usize) -> T {
		// Let it crash if the resource is not found
		self.resources.remove(&id).unwrap()
	}
}

impl<T> Index<usize> for ResourceManager<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		&self.resources[&index]
	}
}

impl<T> IndexMut<usize> for ResourceManager<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.resources.get_mut(&index).unwrap()
	}
}

pub mod date {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn now() -> f64 {
		use std::time::SystemTime;
		let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|e| panic!("{}", e));
		time.as_secs_f64()
	}

	#[cfg(target_arch = "wasm32")]
	pub fn now() -> f64 {
		web_sys::js_sys::Date::now() / 1000.0
	}
}

pub type Context = dyn RenderingBackend;

use std::sync::{Mutex, OnceLock};

static NATIVE_DISPLAY: OnceLock<Mutex<native::NativeDisplayData>> = OnceLock::new();

fn set_display(display: native::NativeDisplayData) {
	if NATIVE_DISPLAY.set(Mutex::new(display)).is_err() {
		panic!("Unable to initialize NATIVE_DISPLAY");
	};
}

fn native_display() -> &'static Mutex<native::NativeDisplayData> {
	NATIVE_DISPLAY.get().expect("Backend has not initialized NATIVE_DISPLAY yet.")
}

/// Window and associated to window rendering context related functions.
/// in macroquad <= 0.3, it was ctx.screen_size(). Now it is window::screen_size()
pub mod window {
	use super::*;

	pub fn new_rendering_backend() -> Box<dyn RenderingBackend> {
		Box::new(GlContext::new())
	}

	/// The current framebuffer size in pixels.
	/// If [`Conf::high_dpi``](crate::conf::Conf::high_dpi) was set to false, canvas|window size == framebuffer size.
	/// If set to true, the framebuffer is scaled by the dpi.
	pub fn screen_size() -> (u32, u32) {
		let d = native_display().lock().unwrap();
		(d.screen_width as u32, d.screen_height as u32)
	}

	/// The dpi scaling factor (window pixels to framebuffer pixels)
	/// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
	pub fn dpi_scale() -> f32 {
		let d = native_display().lock().unwrap();
		d.dpi_scale
	}

	/// True when high_dpi was requested and actually running in a high-dpi scenario
	/// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
	pub fn high_dpi() -> bool {
		let d = native_display().lock().unwrap();
		d.high_dpi
	}

	pub fn blocking_event_loop() -> bool {
		let d = native_display().lock().unwrap();
		d.blocking_event_loop
	}

	/// This function simply quits the application without
	/// giving the user a chance to intervene. Usually this might
	/// be called when the user clicks the 'Ok' button in a 'Really Quit?'
	/// dialog box
	/// Window might not be actually closed right away (exit(0) might not
	/// happen in the order_quit implmentation) and execution might continue for some time after
	/// But the window is going to be inevitably closed at some point.
	pub fn quit() {
		let mut d = native_display().lock().unwrap();
		d.quit = true;
	}

	/// Capture mouse cursor to the current window
	/// On WASM this will automatically hide cursor
	/// On desktop this will bound cursor to windows border
	/// NOTICE: on desktop cursor will not be automatically released after window lost focus
	///         so set_cursor_grab(false) on window's focus lost is recommended.
	/// TODO: implement window focus events
	pub fn set_cursor_grab(grab: bool) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::SetCursorGrab(grab)).unwrap();
	}

	/// With `conf.platform.blocking_event_loop`, `schedule_update` called from an
	/// event handler makes draw()/update() functions to be called without waiting
	/// for a next event.
	///
	/// Does nothing if `conf.platform.blocking_event_loop` == `false`.
	pub fn schedule_update() {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::ScheduleUpdate).unwrap();
	}

	/// Show or hide the mouse cursor
	pub fn show_mouse(shown: bool) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::ShowMouse(shown)).unwrap();
	}

	/// Set the mouse cursor icon.
	pub fn set_mouse_cursor(cursor_icon: CursorIcon) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::SetMouseCursor(cursor_icon)).unwrap();
	}

	/// Set the application's window size.
	pub fn set_window_size(new_width: u32, new_height: u32) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::SetWindowSize { new_width, new_height }).unwrap();
	}

	pub fn set_window_position(new_x: u32, new_y: u32) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::SetWindowPosition { new_x, new_y }).unwrap();
	}

	/// Get the position of the window.
	/// TODO: implement for other platforms
	#[cfg(target_os = "windows")]
	pub fn get_window_position() -> (u32, u32) {
		let d = native_display().lock().unwrap();
		d.screen_position
	}

	pub fn set_fullscreen(fullscreen: bool) {
		let d = native_display().lock().unwrap();
		d.native_requests.send(native::Request::SetFullscreen(fullscreen)).unwrap();
	}

	/// Get current OS clipboard value
	pub fn clipboard_get() -> Option<String> {
		let mut d = native_display().lock().unwrap();
		d.clipboard.get()
	}

	/// Save value to OS clipboard
	pub fn clipboard_set(data: &str) {
		let mut d = native_display().lock().unwrap();
		d.clipboard.set(data)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum CursorIcon {
	Default,
	Help,
	Pointer,
	Wait,
	Crosshair,
	Text,
	Move,
	NotAllowed,
	EWResize,
	NSResize,
	NESWResize,
	NWSEResize,
}

/// Start miniquad_wasm_bindgen.
pub fn start<F>(conf: conf::Conf, f: F)
where
	F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
	#[cfg(target_os = "linux")]
	{
		let mut f = Some(f);
		let f = &mut f;
		match conf.platform.linux_backend {
			conf::LinuxBackend::X11Only => native::linux_x11::run(&conf, f).expect("X11 backend failed"),
			conf::LinuxBackend::WaylandOnly => native::linux_wayland::run(&conf, f).expect("Wayland backend failed"),
			conf::LinuxBackend::X11WithWaylandFallback => {
				if native::linux_x11::run(&conf, f).is_none() {
					#[cfg(feature = "log-impl")]
					crate::error!("Failed to initialize through X11! Trying wayland instead");
					native::linux_wayland::run(&conf, f);
				}
			}
			conf::LinuxBackend::WaylandWithX11Fallback => {
				if native::linux_wayland::run(&conf, f).is_none() {
					#[cfg(feature = "log-impl")]
					crate::error!("Failed to initialize through wayland! Trying X11 instead");
					native::linux_x11::run(&conf, f);
				}
			}
		}
	}

	#[cfg(target_arch = "wasm32")]
	native::wasm::run(&conf, f);

	#[cfg(target_os = "windows")]
	native::windows::run(&conf, f);
}
