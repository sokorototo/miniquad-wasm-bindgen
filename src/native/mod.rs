#![allow(dead_code)]

use std::sync::mpsc;

#[derive(Default)]
pub(crate) struct DroppedFiles {
	pub paths: Vec<std::path::PathBuf>,
	pub bytes: Vec<Vec<u8>>,
}
pub(crate) struct NativeDisplayData {
	pub screen_width: u32,
	pub screen_height: u32,
	pub screen_position: (u32, u32),
	pub dpi_scale: f32,
	pub high_dpi: bool,
	pub quit: bool,
	pub native_requests: mpsc::Sender<Request>,
	pub clipboard: Box<dyn Clipboard>,
	pub dropped_files: DroppedFiles,
	pub blocking_event_loop: bool,
}

impl NativeDisplayData {
	pub fn new(screen_width: u32, screen_height: u32, native_requests: mpsc::Sender<Request>, clipboard: Box<dyn Clipboard>) -> NativeDisplayData {
		NativeDisplayData {
			screen_width,
			screen_height,
			screen_position: (0, 0),
			dpi_scale: 1.,
			high_dpi: false,
			quit: false,
			native_requests,
			clipboard,
			dropped_files: Default::default(),
			blocking_event_loop: false,
		}
	}
}

#[derive(Debug)]
pub(crate) enum Request {
	ScheduleUpdate,
	SetCursorGrab(bool),
	ShowMouse(bool),
	SetMouseCursor(crate::CursorIcon),
	SetWindowSize {
		new_width: u32,
		new_height: u32,
	},
	SetWindowPosition {
		new_x: u32,
		new_y: u32,
	},
	SetFullscreen(bool),
	#[allow(unused)]
	ShowKeyboard(bool),
}

pub trait Clipboard: Send + Sync {
	fn get(&mut self) -> Option<String>;
	fn set(&mut self, string: &str);
}

pub mod module;

#[cfg(target_os = "linux")]
pub mod linux_x11;

#[cfg(target_os = "linux")]
pub mod linux_wayland;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_os = "linux")]
pub mod egl;

// there is no glGetProcAddr on webgl, so its impossible to make "gl" module work
// on macos.. well, there is, but way easier to just statically link to gl
#[cfg(not(target_arch = "wasm32"))]
pub mod gl;

#[cfg(target_arch = "wasm32")]
pub use wasm::webgl as gl;

pub mod query_stab;
