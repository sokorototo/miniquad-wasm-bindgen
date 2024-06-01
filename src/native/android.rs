/// This implementation will be way heavier than the original miniquad solution, since it uses 
/// [`android_activity`](https://docs.rs/android-activity/0.6.0/android_activity/index.html) instead.
/// 
/// It's unfortunately unavoidable, if `miniquad-wasm-bindgen` wants to be highly portable with well maintained crates.
/// This gives more freedom to the end user as well.
/// 
/// ! Important! This implementation uses native_activity. Changed to make it more customizable can be made in the future.


use crate::{
	event::{EventHandler, KeyCode, KeyMods, TouchPhase},
	native::{
		egl::{self, LibEgl}, NativeDisplayData
	},
};

use std::{cell::RefCell, sync::mpsc, thread};

pub use crate::native::gl::{self, *};

mod keycodes;

use android_activity::{AndroidApp, MainEvent, PollEvent, WindowManagerFlags};
use jni::{objects::{JObject, JValue}, AttachGuard, JavaVM};
use ndk::{event::{InputEvent, KeyAction}, native_activity::NativeActivity, native_window::NativeWindow};
use libc::c_void;
pub use ndk;

pub mod ndk_utils;

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
	SurfaceChanged { window: *mut ndk_sys::ANativeWindow, width: i32, height: i32 },
	SurfaceCreated { window: *mut ndk_sys::ANativeWindow },
	SurfaceDestroyed,
	Touch { phase: TouchPhase, touch_id: u64, x: f32, y: f32 },
	Character { character: u32 },
	KeyDown { keycode: KeyCode },
	KeyUp { keycode: KeyCode },
	Pause,
	Resume,
	Destroy,
}
unsafe impl Send for Message {}

thread_local! {
	static MESSAGES_TX: RefCell<Option<mpsc::Sender<Message>>> = RefCell::new(None);
}

fn send_message(message: Message) {
	MESSAGES_TX.with(|tx| {
		let mut tx = tx.borrow_mut();
		tx.as_mut().unwrap().send(message).unwrap();
	})
}

static mut ACTIVITY: Option<*mut c_void> = None;
static mut VM: Option<*mut c_void> = None;
static mut ANDROID_APP: Option<AndroidApp> = None; 

unsafe fn attach_current_thread<'a>(vm: &'a JavaVM) -> AttachGuard<'a> {
	vm.attach_current_thread().expect("Failed to attach JavaVM to the current thread")
}

unsafe fn get_current_vm() -> JavaVM {
	let vm = VM.expect("JavaVM should be avaiable before process_request");
	JavaVM::from_raw(vm as _).unwrap()
}

unsafe fn get_current_activity() -> *mut c_void {
	ACTIVITY.expect("Activity is None at this moment of runtime")
}

pub unsafe fn console_debug(msg: *const ::std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_DEBUG.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_info(msg: *const ::std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_INFO.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_warn(msg: *const ::std::os::raw::c_char) {
	ndk_sys::__android_log_write(ndk_sys::android_LogPriority::ANDROID_LOG_WARN.0 as _, b"SAPP\0".as_ptr() as _, msg);
}

pub unsafe fn console_error(msg: *const ::std::os::raw::c_char) {
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

	fn process_message(&mut self, msg: Message) {
		match msg {
			Message::SurfaceCreated { window } => unsafe {
				self.update_surface(window);
			},
			Message::SurfaceDestroyed => unsafe {
				self.destroy_surface();
			},
			Message::SurfaceChanged { window, width, height } => {
				unsafe {
					self.update_surface(window);
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
					// let env = VM.expect("JavaVM should be avaiable before process_request")
					// 	.attach_current_thread()
					// 	.expect("Failed to attach JavaVM to current thread");
					// unsafe {
					// 	set_fullscreen(env, true);
					// }
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

	fn process_request(&mut self, request: crate::native::Request) {
		use crate::native::Request::*;
		unsafe {
			match request {
				SetFullscreen(fullscreen) => {
					let vm = get_current_vm();
					let mut env = attach_current_thread(&vm);
					set_fullscreen(&mut env, fullscreen);
					self.fullscreen = fullscreen;
				}
				ShowKeyboard(show) => {
					if let Some(activity) = ACTIVITY {
						let vm = {
							let vm = VM.expect("JavaVM should be avaiable before process_request");
							JavaVM::from_raw(vm as _).unwrap()
						};
						let mut env = vm.attach_current_thread().expect("Failed to attach JavaVM to current thread");
						env.call_method(JObject::from_raw(activity as _), "showKeyboard", "(Z)V", &[JValue::Int(show as _)]);
					}
				},
				_ => {}
			}
		}
	}
}

/// Get the JNI Env by calling ndk's AttachCurrentThread
///
/// Safety note: This function is not exactly correct now, it should be fixed!
///
/// AttachCurrentThread should be called at least once for any given thread that
/// wants to use the JNI and DetachCurrentThread should be called only once, when
/// the thread stack is empty and the thread is about to stop
///
/// calling AttachCurrentThread from the same thread multiple time is very cheap
///
/// BUT! there is no DetachCurrentThread call right now, this code:
/// `thread::spawn(|| attach_jni_env());` will lead to internal jni crash :/
/// thread::spawn(|| { attach_jni_env(); loop {} }); is basically what miniquad_wasm_bindgen
/// is doing. this is not correct, but works
/// TODO: the problem here -
/// TODO:   thread::spawn(|| { Attach(); .. Detach() }); will not work as well.
/// TODO: JNI will check that thread's stack is still alive and will crash.
///
/// TODO: Figure how to get into the thread destructor to correctly call Detach
/// TODO: (this should be a GH issue)
/// TODO: for reference - grep for "pthread_setspecific" in SDL2 sources, SDL fixed it!
// pub unsafe fn attach_jni_env() -> *mut jni_sys::JNIEnv {
// 	let mut env: *mut jni_sys::JNIEnv = std::ptr::null_mut();
// 	let attach_current_thread = (**VM).AttachCurrentThread.unwrap();

// 	let res = attach_current_thread(VM, env as *mut _, std::ptr::null_mut());
// 	assert!(res == 0);

// 	env
// }

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

/// Initialization method that should be called before [`miniquad::start`](https://docs.rs/miniquad/latest/miniquad/fn.start.html).
/// Mobile development on rust has evolved, and now windowing crates should require the user provided `android_activity` [`AndroidApp`] handle
/// to manage said activities.
/// 
/// Only use this function from the main thread.
pub fn init_android_activity(app: AndroidApp) {
	unsafe {
		ANDROID_APP = Some(app);
	}
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
	
	let app = ANDROID_APP.clone().expect("init_quad_activity should be run before miniquad::start on android");

	unsafe {
		// Initialize the ACTIVITY and VM pointer
		ACTIVITY = Some(app.activity_as_ptr());
		VM = Some(app.vm_as_ptr());
	}

	if conf.fullscreen {
		// TODO: Implement fullscreen
		let vm = get_current_vm();
		let mut env = attach_current_thread(&vm);
		set_fullscreen(&mut env, true);
	}

	// yeah, just adding Send to outer F will do it, but it will break the API on other backends
	struct SendHack<F>(F);
	unsafe impl<F> Send for SendHack<F> {}

	let f = SendHack(f);

	let (tx, rx) = mpsc::channel();

	MESSAGES_TX.with(move |messages_tx| *messages_tx.borrow_mut() = Some(tx));

	let mut libegl = LibEgl::try_load().expect("Cant load LibEGL");

	// skip all the messages until android will be able to actually open a window
	//
	// sometimes before launching an app android will show a permission dialog
	// it is important to create GL context only after a first SurfaceChanged
	let (window, screen_width, screen_height) = loop {
		let mut display_avaiable = false;
		app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
			match event {
				PollEvent::Main(main_event) => {
					// log::info!("Main event: {:?}", main_event);
					match main_event {
						MainEvent::Destroy => { return; },
						MainEvent::InitWindow { .. } => { display_avaiable = true; }
						_ => {}
					}
				},
				_ => {}
			}
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

	let event_handler = f.0();
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

	while !s.quit {
		while let Ok(request) = requests_rx.try_recv() {
			s.process_request(request);
		}

		// ! 16 millis here is a magic number that should be changed
		app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
			match event {
				PollEvent::Main(main_event) => {
					// log::info!("Main event: {:?}", main_event);
					match main_event {
						MainEvent::Destroy => { 
							s.process_message(Message::Destroy);
						},
						MainEvent::Pause => {
							s.process_message(Message::Pause);
						},
						MainEvent::Resume { loader, .. } => {
							s.process_message(Message::Resume);
						},
						MainEvent::InitWindow { .. } => {
							let wnd = app.native_window().unwrap();
							s.process_message(Message::SurfaceCreated { 
								window: wnd.ptr().as_ptr()
							});
						},
						MainEvent::WindowResized { .. } => {
							let wnd = app.native_window().unwrap();
							s.process_message(Message::SurfaceChanged { 
								window: wnd.ptr().as_mut(), 
								width: wnd.width(), 
								height: wnd.height() 
							});
						}
						_ => {}
					}
				},
				_ => {}
			};
		});

		s.frame();

		thread::yield_now();
	}

	(s.libegl.eglMakeCurrent.unwrap())(s.egl_display, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
	(s.libegl.eglDestroySurface.unwrap())(s.egl_display, s.surface);
	(s.libegl.eglDestroyContext.unwrap())(s.egl_display, s.egl_context);
	(s.libegl.eglTerminate.unwrap())(s.egl_display);
}

// #[no_mangle]
// extern "C" fn jni_on_load(vm: *mut std::ffi::c_void) {
// 	unsafe {
// 		VM = vm as _;
// 	}
// }

// unsafe fn create_native_window(surface: ndk_sys::jobject) -> *mut ndk_sys::ANativeWindow {
// 	let env = attach_jni_env();

// 	ndk_sys::ANativeWindow_fromSurface(env, surface)
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_quad_1native_QuadNative_activityOnCreate(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, activity: ndk_sys::jobject) {
// 	let env = attach_jni_env();
// 	ACTIVITY = (**env).NewGlobalRef.unwrap()(env, activity);
// 	quad_main();
// }

// #[no_mangle]
// unsafe extern "C" fn Java_quad_1native_QuadNative_activityOnResume(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject) {
// 	send_message(Message::Resume);
// }

// #[no_mangle]
// unsafe extern "C" fn Java_quad_1native_QuadNative_activityOnPause(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject) {
// 	send_message(Message::Pause);
// }

// #[no_mangle]
// unsafe extern "C" fn Java_quad_1native_QuadNative_activityOnDestroy(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject) {
// 	send_message(Message::Destroy);
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnSurfaceCreated(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, surface: ndk_sys::jobject) {
// 	let window = unsafe { create_native_window(surface) };
// 	send_message(Message::SurfaceCreated { window });
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnSurfaceDestroyed(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject) {
// 	send_message(Message::SurfaceDestroyed);
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnSurfaceChanged(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, surface: ndk_sys::jobject, width: ndk_sys::jint, height: ndk_sys::jint) {
// 	let window = unsafe { create_native_window(surface) };

// 	send_message(Message::SurfaceChanged {
// 		window,
// 		width: width as _,
// 		height: height as _,
// 	});
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnTouch(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, touch_id: ndk_sys::jint, action: ndk_sys::jint, x: ndk_sys::jfloat, y: ndk_sys::jfloat) {
// 	let phase = match action {
// 		0 => TouchPhase::Moved,
// 		1 => TouchPhase::Ended,
// 		2 => TouchPhase::Started,
// 		3 => TouchPhase::Cancelled,
// 		x => panic!("Unsupported touch phase: {}", x),
// 	};

// 	send_message(Message::Touch {
// 		phase,
// 		touch_id: touch_id as _,
// 		x: x as f32,
// 		y: y as f32,
// 	});
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnKeyDown(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, keycode: ndk_sys::jint) {
// 	let keycode = keycodes::translate_keycode(keycode as _);

// 	send_message(Message::KeyDown { keycode });
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnKeyUp(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, keycode: ndk_sys::jint) {
// 	let keycode = keycodes::translate_keycode(keycode as _);

// 	send_message(Message::KeyUp { keycode });
// }

// #[no_mangle]
// extern "C" fn Java_quad_1native_QuadNative_surfaceOnCharacter(_: *mut ndk_sys::JNIEnv, _: ndk_sys::jobject, character: ndk_sys::jint) {
// 	send_message(Message::Character { character: character as u32 });
// }

unsafe fn set_fullscreen(env: &mut AttachGuard<'_>, fullscreen: bool) {
	let activity = JObject::from_raw(get_current_activity() as _);
	env.call_method(activity, "setFullScreen", "(Z)V", &[JValue::Int(fullscreen as i32)]);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AndroidAsset {
	pub content: *mut ::std::os::raw::c_char,
	pub content_length: ::std::os::raw::c_int,
}

// According to documentation, AAssetManager_fromJava is as available as an
// AAssetManager_open, which was used before
// For some reason it is missing fron ndk_sys binding

// TODO: Implement asset loading
// extern "C" {
// 	pub fn AAssetManager_fromJava(env: *mut ndk_sys::JNIEnv, assetManager: ndk_sys::jobject) -> *mut ndk_sys::AAssetManager;
// }

// pub(crate) unsafe fn load_asset(filepath: *const ::std::os::raw::c_char, out: *mut AndroidAsset) {
// 	let env = attach_jni_env();

// 	let get_method_id = (**env).GetMethodID.unwrap();
// 	let get_object_class = (**env).GetObjectClass.unwrap();
// 	let call_object_method = (**env).CallObjectMethod.unwrap();

// 	let mid = (get_method_id)(env, get_object_class(env, ACTIVITY), b"getAssets\0".as_ptr() as _, b"()Landroid/content/res/AssetManager;\0".as_ptr() as _);
// 	let asset_manager = (call_object_method)(env, ACTIVITY, mid);
// 	let mgr = AAssetManager_fromJava(env, asset_manager);
// 	let asset = ndk_sys::AAssetManager_open(mgr, filepath, ndk_sys::AASSET_MODE_BUFFER as _);
// 	if asset.is_null() {
// 		return;
// 	}
// 	let length = ndk_sys::AAsset_getLength64(asset);
// 	// TODO: memory leak right here! this buffer would never freed
// 	let buffer = libc::malloc(length as _);
// 	if ndk_sys::AAsset_read(asset, buffer, length as _) > 0 {
// 		ndk_sys::AAsset_close(asset);

// 		(*out).content_length = length as _;
// 		(*out).content = buffer as _;
// 	}
// }
