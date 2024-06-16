use std::{ffi::CStr, io::Read, sync::OnceLock};

/// This implementation will be way heavier than the original miniquad solution, since it uses
/// [`android_activity`](https://docs.rs/android-activity/0.6.0/android_activity/index.html).
///
/// It's unfortunately unavoidable, if `miniquad-wasm-bindgen` wants to be highly portable with well maintained crates.
/// This though, gives more freedom to the end user.
///
/// **Important! This implementation uses the android native activity. Changes to make it more customizable can be made in the future.**
use crate::{
	event::{EventHandler, KeyCode, KeyMods, TouchPhase},
	native::{
		android::keycodes::translate_keycode_ndk,
		egl::{self, LibEgl},
		NativeDisplayData,
	},
};

mod keycodes;

use android_activity::input::InputEvent;
use android_activity::{
	input::{KeyAction, MotionAction},
	AndroidApp, InputStatus, MainEvent, PollEvent, WindowManagerFlags,
};
use jni::{
	objects::{JObject, JValue},
	AttachGuard, JavaVM,
};
use libc::c_void;
use ndk::asset::AssetManager;

/// Short recap on how miniquad_wasm_bindgen on Android works
/// There is a MainActivity, a normal Java activity
/// It creates a View and pass a reference to a view to rust.
/// Rust spawn a thread that render things into this view as often as
/// possible.
/// Also MainActivty collects user input events and calls native rust functions.
///
/// This long explanation was to illustrate how we ended up with evets callback
/// and drawing in the different threads.
/// Message enum is used to send data from the callbacks to the drawing thread.
#[derive(Debug)]
enum Message {
	SurfaceChanged,
	SurfaceCreated,
	SurfaceDestroyed,
	Touch { phase: TouchPhase, touch_id: u64, x: f32, y: f32 },
	Character { character: u32 },
	KeyDown { keycode: KeyCode },
	KeyUp { keycode: KeyCode },
	Pause,
	Resume,
	Destroy,
}

static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();
static ASSET_MANAGER: OnceLock<AssetManager> = OnceLock::new();

/// Initialization method that should be called before [`miniquad::start`](https://docs.rs/miniquad/latest/miniquad/fn.start.html).
/// Mobile development on rust has evolved, and now windowing crates should require the user provided `android_activity` [`AndroidApp`] handle
/// to manage said activities.
///
/// Only use this function from the main thread
pub fn init_android_activity(app: AndroidApp) {
	let _ = ANDROID_APP.set(app);
}

unsafe fn attach_current_thread<'a>(vm: &'a JavaVM) -> AttachGuard<'a> {
	vm.attach_current_thread().expect("Failed to attach JavaVM to the current thread")
}

pub unsafe fn console_debug(msg: *const std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_info(msg: *const std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_INFO.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_warn(msg: *const std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_WARN.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_error(msg: *const std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_ERROR.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

struct MainThreadState {
	libegl: LibEgl,
	egl_display: egl::EGLDisplay,
	egl_config: egl::EGLConfig,
	egl_context: egl::EGLContext,
	surface: egl::EGLSurface,
	window: *mut ndk_sys::ANativeWindow,
	event_handler: Box<dyn EventHandler>,
	quit: bool,
	fullscreen: bool,
	keymods: KeyMods,
}

impl MainThreadState {
	unsafe fn destroy_surface(&mut self) {
		(self.libegl.eglMakeCurrent.unwrap())(self.egl_display, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
		(self.libegl.eglDestroySurface.unwrap())(self.egl_display, self.surface);
		self.surface = std::ptr::null_mut();
	}

	unsafe fn update_surface(&mut self, window: *mut ndk_sys::ANativeWindow) {
		if !self.window.is_null() {
			ndk_sys::ANativeWindow_release(self.window);
		}
		self.window = window;
		if self.surface.is_null() == false {
			self.destroy_surface();
		}

		self.surface = (self.libegl.eglCreateWindowSurface.unwrap())(self.egl_display, self.egl_config, window as _, std::ptr::null_mut());

		assert!(!self.surface.is_null());

		let res = (self.libegl.eglMakeCurrent.unwrap())(self.egl_display, self.surface, self.surface, self.egl_context);

		assert!(res != 0);
	}

	fn process_message(&mut self, app: &AndroidApp, msg: Message) {
		match msg {
			Message::SurfaceCreated => {
				let wnd = app.native_window().unwrap();

				unsafe {
					self.update_surface(wnd.ptr().as_ptr());
				}
			}
			Message::SurfaceDestroyed => unsafe {
				self.destroy_surface();
			},
			Message::SurfaceChanged => {
				let wnd = app.native_window().unwrap();
				let (width, height) = (wnd.width(), wnd.height());

				unsafe {
					self.update_surface(wnd.ptr().as_ptr());
				}

				{
					let mut d = crate::native_display().lock().unwrap();
					d.screen_width = width as _;
					d.screen_height = height as _;
				}
				self.event_handler.resize_event(width as _, height as _);
			}
			Message::Touch { phase, touch_id, x, y } => {
				self.event_handler.touch_event(phase, touch_id, x, y);
			}
			Message::Character { character } => {
				if let Some(character) = char::from_u32(character) {
					self.event_handler.char_event(character, Default::default(), false);
				}
			}
			Message::KeyDown { keycode } => {
				match keycode {
					KeyCode::LeftShift | KeyCode::RightShift => self.keymods.shift = true,
					KeyCode::LeftControl | KeyCode::RightControl => self.keymods.ctrl = true,
					KeyCode::LeftAlt | KeyCode::RightAlt => self.keymods.alt = true,
					KeyCode::LeftSuper | KeyCode::RightSuper => self.keymods.logo = true,
					_ => {}
				}
				self.event_handler.key_down_event(keycode, self.keymods, false);
			}
			Message::KeyUp { keycode } => {
				match keycode {
					KeyCode::LeftShift | KeyCode::RightShift => self.keymods.shift = false,
					KeyCode::LeftControl | KeyCode::RightControl => self.keymods.ctrl = false,
					KeyCode::LeftAlt | KeyCode::RightAlt => self.keymods.alt = false,
					KeyCode::LeftSuper | KeyCode::RightSuper => self.keymods.logo = false,
					_ => {}
				}
				self.event_handler.key_up_event(keycode, self.keymods);
			}
			Message::Pause => self.event_handler.window_minimized_event(),
			Message::Resume => {
				if self.fullscreen {
					unsafe {
						let vm = JavaVM::from_raw(app.vm_as_ptr() as _).expect("Android App's vm pointer should be valid");
						let mut env = attach_current_thread(&vm);
						set_fullscreen(app.activity_as_ptr(), &mut env, true);
					}
				}

				self.event_handler.window_restored_event()
			}
			Message::Destroy => {
				self.quit = true;
			}
		}
	}

	fn frame(&mut self) {
		self.event_handler.update();

		if self.surface.is_null() == false {
			self.event_handler.draw();

			unsafe {
				(self.libegl.eglSwapBuffers.unwrap())(self.egl_display, self.surface);
			}
		}
	}

	unsafe fn process_request(&mut self, vm: &mut JavaVM, activity: *mut c_void, request: crate::native::Request) {
		use crate::native::Request;
		match request {
			Request::SetFullscreen(fullscreen) => {
				let mut env = attach_current_thread(&vm);
				set_fullscreen(activity, &mut env, fullscreen);
				self.fullscreen = fullscreen;
			}
			Request::ShowKeyboard(show) => {
				// let mut env = vm.attach_current_thread().expect("Failed to attach JavaVM to current thread");
				// let _ = env.call_method(JObject::from_raw(activity as _), "showKeyboard", "(Z)V", &[JValue::Int(show as _)]);
			}
			_ => {}
		}
	}
}

pub struct AndroidClipboard {}
impl AndroidClipboard {
	pub fn new() -> AndroidClipboard {
		AndroidClipboard {}
	}
}
impl crate::native::Clipboard for AndroidClipboard {
	fn get(&mut self) -> Option<String> {
		None
	}

	fn set(&mut self, data: &str) {}
}

pub unsafe fn run<F>(conf: crate::conf::Conf, f: F)
where
	F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
	{
		use std::ffi::CString;
		use std::panic;

		panic::set_hook(Box::new(|info| {
			let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap());
			console_error(msg.as_ptr());
		}));
	}

	let app = ANDROID_APP.get().expect("init_android_activity should be run before miniquad::start on android");
	let _ = ASSET_MANAGER.set(app.asset_manager());

	let mut vm = unsafe { JavaVM::from_raw(app.vm_as_ptr() as _).expect("VM pointer should be valid") };
	let activity = app.activity_as_ptr();

	app.set_window_flags(WindowManagerFlags::FULLSCREEN, WindowManagerFlags::empty());
	if conf.fullscreen {
		// TODO: Hide the navbar as well
		let mut env = attach_current_thread(&vm);
		set_fullscreen(activity, &mut env, true);
	}

	let mut libegl = LibEgl::try_load().expect("Cant load LibEGL");

	// skip all the messages until android will be able to actually open a window
	//
	// sometimes before launching an app android will show a permission dialog
	// it is important to create GL context only after a first SurfaceChanged
	let (window, screen_width, screen_height) = loop {
		let mut display_avaiable = false;
		app.poll_events(Some(std::time::Duration::from_millis(16)), |event| match event {
			PollEvent::Main(main_event) => match main_event {
				MainEvent::Destroy => {
					return;
				}
				MainEvent::InitWindow { .. } => {
					display_avaiable = true;
				}
				_ => {}
			},
			_ => {}
		});

		if display_avaiable {
			let wnd = app.native_window().expect("Window should be avaiable after MainEvent::InitWindow");
			let (width, height) = (wnd.width(), wnd.height());
			let window = wnd.ptr().as_ptr();
			break (window, width, height);
		}
	};

	let (egl_context, egl_config, egl_display) =
		crate::native::egl::create_egl_context(&mut libegl, std::ptr::null_mut() /* EGL_DEFAULT_DISPLAY */, conf.platform.framebuffer_alpha, conf.sample_count).expect("Cant create EGL context");

	assert!(!egl_display.is_null());
	assert!(!egl_config.is_null());

	crate::native::gl::load_gl_funcs(|proc| {
		let name = std::ffi::CString::new(proc).unwrap();
		libegl.eglGetProcAddress.expect("non-null function pointer")(name.as_ptr() as _)
	});

	let surface = (libegl.eglCreateWindowSurface.unwrap())(egl_display, egl_config, window as _, std::ptr::null_mut());

	if (libegl.eglMakeCurrent.unwrap())(egl_display, surface, surface, egl_context) == 0 {
		panic!();
	}

	let (tx, requests_rx) = std::sync::mpsc::channel();
	let clipboard = Box::new(AndroidClipboard::new());
	crate::set_display(NativeDisplayData {
		high_dpi: conf.high_dpi,
		..NativeDisplayData::new(screen_width as _, screen_height as _, tx, clipboard)
	});

	let event_handler = f();
	let mut s = MainThreadState {
		libegl,
		egl_display,
		egl_config,
		egl_context,
		surface,
		window,
		event_handler,
		quit: false,
		fullscreen: conf.fullscreen,
		keymods: KeyMods {
			shift: false,
			ctrl: false,
			alt: false,
			logo: false,
		},
	};
	let mut messages: Vec<Message> = Vec::with_capacity(100);
	let mut input_avaiable = false;
	while !s.quit {
		while let Ok(request) = requests_rx.try_recv() {
			s.process_request(&mut vm, activity, request);
		}

		// ! 16 millis here is a magic number that should be changed
		app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
			match event {
				PollEvent::Main(main_event) => match main_event {
					MainEvent::Destroy => {
						messages.push(Message::Destroy);
					}
					MainEvent::Pause => {
						messages.push(Message::Pause);
					}
					MainEvent::Resume { .. } => {
						messages.push(Message::Resume);
					}
					MainEvent::InitWindow { .. } => {
						messages.push(Message::SurfaceCreated);
					}
					MainEvent::WindowResized { .. } => {
						messages.push(Message::SurfaceChanged);
					}
					MainEvent::TerminateWindow { .. } => {
						messages.push(Message::SurfaceDestroyed);
					}
					MainEvent::InputAvailable => {
						input_avaiable = true;
					}
					_ => {}
				},
				_ => {}
			};
		});

		if input_avaiable {
			input_avaiable = false;
			if let Ok(mut iter) = app.input_events_iter() {
				loop {
					let read_input = iter.next(|event| {
						match event {
							InputEvent::KeyEvent(key_event) => {
								let keycode = key_event.key_code();
								match key_event.action() {
									KeyAction::Down => {
										messages.push(Message::KeyDown {
											keycode: translate_keycode_ndk(keycode),
										});
									}
									KeyAction::Up | KeyAction::Multiple => {
										messages.push(Message::KeyUp {
											keycode: translate_keycode_ndk(keycode),
										});
									}
									_ => (),
								};
							}
							InputEvent::MotionEvent(motion_event) => {
								let ind = motion_event.pointer_index();
								let ptr = motion_event.pointer_at_index(ind);
								// TODO: It seems like MotionEvent can also come from java action UI interactions. Im ignoring them here
								let phase = match motion_event.action() {
									MotionAction::Cancel | MotionAction::Outside => Some(TouchPhase::Cancelled),
									MotionAction::PointerDown | MotionAction::Down => Some(TouchPhase::Started),
									MotionAction::PointerUp | MotionAction::Up => Some(TouchPhase::Ended),
									MotionAction::Move | MotionAction::Scroll => Some(TouchPhase::Moved),
									_ => None,
								};
								if let Some(phase) = phase {
									// TODO: Touch event here is bugged, can't grab multiple pointers currently.
									messages.push(Message::Touch {
										phase,
										touch_id: ptr.pointer_index() as u64,
										x: ptr.x(),
										y: ptr.y(),
									});
								}
							},
							InputEvent::TextEvent(text) => {},
							_ => {}
						};
						InputStatus::Handled
					});
					if !read_input {
						break;
					}
				}
			}
		}

		for message in messages.drain(..) {
			s.process_message(&app, message);
		}

		s.frame();
	}

	(s.libegl.eglMakeCurrent.unwrap())(s.egl_display, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
	(s.libegl.eglDestroySurface.unwrap())(s.egl_display, s.surface);
	(s.libegl.eglDestroyContext.unwrap())(s.egl_display, s.egl_context);
	(s.libegl.eglTerminate.unwrap())(s.egl_display);
}

unsafe fn set_fullscreen(activity: *mut c_void, env: &mut AttachGuard<'_>, fullscreen: bool) {
	let activity = JObject::from_raw(activity as _);
	let _ = env.call_method(activity, "setFullScreen", "(Z)V", &[JValue::Int(fullscreen as i32)]);
}

// According to documentation, AAssetManager_fromJava is as available as an
// AAssetManager_open, which was used before
// For some reason it is missing fron ndk_sys binding

pub(crate) fn load_asset(filepath: &CStr) -> Option<Vec<u8>> {
	let manager = ASSET_MANAGER.get().expect("Asset Manager should be initialised before loading assets");

	match manager.open(filepath) {
		Some(mut asset) => {
			let mut buff: Vec<u8> = Vec::new();
			match asset.read_to_end(&mut buff) {
				Ok(_) => Some(buff),
				Err(_) => {
					#[cfg(feature = "log-impl")]
					unsafe {
						console_warn("File read operation was interrupted!".as_ptr())
					};
					None
				}
			}
		}
		None => {
			#[cfg(feature = "log-impl")]
			unsafe {
				console_warn("No asset found!".as_ptr())
			};
			None
		}
	}
}
