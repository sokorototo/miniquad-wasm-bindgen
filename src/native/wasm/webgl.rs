//! There is no glGetProcAddr on web.
//! The only way to get gl functions - actually tell the linker to link with
//! their gl.js counterparts.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::{collections::BTreeMap, slice};

use wasm_bindgen::*;
use web_sys::*;

pub type GLenum = ::std::os::raw::c_uint;
pub type GLboolean = ::std::os::raw::c_uchar;
pub type GLbitfield = ::std::os::raw::c_uint;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLbyte = ::std::os::raw::c_schar;
pub type GLshort = ::std::os::raw::c_short;
pub type GLint = ::std::os::raw::c_int;
pub type GLubyte = ::std::os::raw::c_uchar;
pub type GLushort = ::std::os::raw::c_ushort;
pub type GLuint = ::std::os::raw::c_uint;
pub type GLint64 = ::std::os::raw::c_longlong;
pub type GLuint64 = ::std::os::raw::c_ulonglong;
pub type GLsizei = ::std::os::raw::c_int;
pub type GLchar = ::std::os::raw::c_char;

pub type khronos_ssize_t = ::std::os::raw::c_long;
pub type khronos_usize_t = ::std::os::raw::c_ulong;
pub type khronos_intptr_t = ::std::os::raw::c_long;

pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;

pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

pub const GL_INT_2_10_10_10_REV: u32 = 0x8D9F;
pub const GL_PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
pub const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
pub const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const GL_COLOR_ATTACHMENT22: u32 = 0x8CF6;
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const GL_NUM_EXTENSIONS: u32 = 0x821D;
pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
pub const GL_SHADER_SOURCE_LENGTH: u32 = 0x8B88;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_INCR: u32 = 0x1E02;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const GL_FUNC_SUBTRACT: u32 = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
pub const GL_CONSTANT_COLOR: u32 = 0x8001;
pub const GL_DECR_WRAP: u32 = 0x8508;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_SHORT: u32 = 0x1402;
pub const GL_DEPTH_TEST: u32 = 0x0B71;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
pub const GL_RGBA16F: u32 = 0x881A;
pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
pub const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const GL_TEXTURE0: u32 = 0x84C0;
pub const GL_TEXTURE_MIN_LOD: u32 = 0x813A;
pub const GL_CLAMP_TO_EDGE: u32 = 0x812F;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const GL_TEXTURE_WRAP_R: u32 = 0x8072;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
pub const GL_STREAM_DRAW: u32 = 0x88E0;
pub const GL_ONE: u32 = 1;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const GL_RGB10_A2: u32 = 0x8059;
pub const GL_RGBA8: u32 = 0x8058;
pub const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
pub const GL_RGBA4: u32 = 0x8056;
pub const GL_RGB8: u32 = 0x8051;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STENCIL: u32 = 0x1802;
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_DEPTH: u32 = 0x1801;
pub const GL_FRONT: u32 = 0x0404;
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const GL_REPEAT: u32 = 0x2901;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const GL_DECR: u32 = 0x1E03;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_TEXTURE_MAX_LOD: u32 = 0x813B;
// TODO: Use exports from WebGl2RenderingContext
pub const GL_DEPTH_COMPONENT: u32 = WebGl2RenderingContext::DEPTH_COMPONENT;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
pub const GL_COLOR: u32 = 0x1800;
pub const GL_TEXTURE_2D_ARRAY: u32 = 0x8C1A;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const GL_NONE: u32 = 0;
pub const GL_SRC_COLOR: u32 = 0x0300;
pub const GL_BYTE: u32 = 0x1400;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
pub const GL_LINE_STRIP: u32 = 0x0003;
pub const GL_TEXTURE_3D: u32 = 0x806F;
pub const GL_CW: u32 = 0x0900;
pub const GL_LINEAR: u32 = 0x2601;
pub const GL_RENDERBUFFER: u32 = 0x8D41;
pub const GL_GEQUAL: u32 = 0x0206;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_RGBA32F: u32 = 0x8814;
pub const GL_BLEND: u32 = 0x0BE2;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const GL_EXTENSIONS: u32 = 0x1F03;
pub const GL_NO_ERROR: u32 = 0;
pub const GL_REPLACE: u32 = 0x1E01;
pub const GL_KEEP: u32 = 0x1E00;
pub const GL_CCW: u32 = 0x0901;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const GL_RGB: u32 = 0x1907;
pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
pub const GL_FALSE: u32 = 0;
pub const GL_ZERO: u32 = 0;
pub const GL_CULL_FACE: u32 = 0x0B44;
pub const GL_INVERT: u32 = 0x150A;
pub const GL_INT: u32 = 0x1404;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_NEAREST: u32 = 0x2600;
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
pub const GL_LEQUAL: u32 = 0x0203;
pub const GL_STENCIL_TEST: u32 = 0x0B90;
pub const GL_DITHER: u32 = 0x0BD0;
pub const GL_DEPTH_COMPONENT16: u32 = 0x81A5;
pub const GL_EQUAL: u32 = 0x0202;
pub const GL_FRAMEBUFFER: u32 = 0x8D40;
pub const GL_RGB5: u32 = 0x8050;
pub const GL_LINES: u32 = 0x0001;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_SRC_ALPHA: u32 = 0x0302;
pub const GL_INCR_WRAP: u32 = 0x8507;
pub const GL_LESS: u32 = 0x0201;
pub const GL_MULTISAMPLE: u32 = 0x809D;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6; // 36006
pub const GL_BACK: u32 = 0x0405;
pub const GL_ALWAYS: u32 = 0x0207;
pub const GL_FUNC_ADD: u32 = 0x8006;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
pub const GL_NOTEQUAL: u32 = 0x0205;
pub const GL_DST_COLOR: u32 = 0x0306;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_DELETE_STATUS: u32 = 0x8B80;
pub const GL_SHADER_TYPE: u32 = 0x8B4F;
pub const GL_ACTIVE_UNIFORMS: u32 = 0x8B86;
pub const GL_ACTIVE_ATTRIBUTES: u32 = 0x8B89;
pub const GL_RED: u32 = 0x1903;
pub const GL_GREEN: u32 = 6404;
pub const GL_BLUE: u32 = 6405;
pub const GL_ALPHA: u32 = 6406;
pub const GL_LUMINANCE: u32 = 6409;
pub const GL_LUMINANCE_ALPHA: u32 = 6410;
pub const GL_ALPHA_BITS: u32 = 3413;
pub const GL_RED_BITS: u32 = 3410;
pub const GL_GREEN_BITS: u32 = 3411;
pub const GL_BLUE_BITS: u32 = 3412;
pub const GL_INDEX_BITS: u32 = 3409;
pub const GL_SUBPIXEL_BITS: u32 = 3408;
pub const GL_AUX_BUFFERS: u32 = 3072;
pub const GL_READ_BUFFER: u32 = 3074;
pub const GL_DRAW_BUFFER: u32 = 3073;
pub const GL_DOUBLEBUFFER: u32 = 3122;
pub const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
pub const GL_DST_ALPHA: u32 = 0x0304;
pub const GL_RGB5_A1: u32 = 0x8057;
pub const GL_GREATER: u32 = 0x0204;
pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const GL_TRUE: u32 = 1;
pub const GL_NEVER: u32 = 0x0200;
pub const GL_POINTS: u32 = 0x0000;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const GL_MIRRORED_REPEAT: u32 = 0x8370;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
pub const GL_R11F_G11F_B10F: u32 = 0x8C3A;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
pub const GL_RGBA32UI: u32 = 0x8D70;
pub const GL_RGB32UI: u32 = 0x8D71;
pub const GL_RGBA16UI: u32 = 0x8D76;
pub const GL_RGB16UI: u32 = 0x8D77;
pub const GL_RGBA8UI: u32 = 0x8D7C;
pub const GL_RGB8UI: u32 = 0x8D7D;
pub const GL_RGBA32I: u32 = 0x8D82;
pub const GL_RGB32I: u32 = 0x8D83;
pub const GL_RGBA16I: u32 = 0x8D88;
pub const GL_RGB16I: u32 = 0x8D89;
pub const GL_RGBA8I: u32 = 0x8D8E;
pub const GL_RGB8I: u32 = 0x8D8F;
pub const GL_RED_INTEGER: u32 = 0x8D94;
pub const GL_RG: u32 = 0x8227;
pub const GL_RG_INTEGER: u32 = 0x8228;
pub const GL_R8: u32 = 0x8229;
pub const GL_R16: u32 = 0x822A;
pub const GL_RG8: u32 = 0x822B;
pub const GL_RG16: u32 = 0x822C;
pub const GL_R16F: u32 = 0x822D;
pub const GL_R32F: u32 = 0x822E;
pub const GL_RG16F: u32 = 0x822F;
pub const GL_RG32F: u32 = 0x8230;
pub const GL_R8I: u32 = 0x8231;
pub const GL_R8UI: u32 = 0x8232;
pub const GL_R16I: u32 = 0x8233;
pub const GL_R16UI: u32 = 0x8234;
pub const GL_R32I: u32 = 0x8235;
pub const GL_R32UI: u32 = 0x8236;
pub const GL_RG8I: u32 = 0x8237;
pub const GL_RG8UI: u32 = 0x8238;
pub const GL_RG16I: u32 = 0x8239;
pub const GL_RG16UI: u32 = 0x823A;
pub const GL_RG32I: u32 = 0x823B;
pub const GL_RG32UI: u32 = 0x823C;
pub const GL_RGBA_INTEGER: u32 = 0x8D99;
pub const GL_R8_SNORM: u32 = 0x8F94;
pub const GL_RG8_SNORM: u32 = 0x8F95;
pub const GL_RGB8_SNORM: u32 = 0x8F96;
pub const GL_RGBA8_SNORM: u32 = 0x8F97;
pub const GL_R16_SNORM: u32 = 0x8F98;
pub const GL_RG16_SNORM: u32 = 0x8F99;
pub const GL_RGB16_SNORM: u32 = 0x8F9A;
pub const GL_RGBA16_SNORM: u32 = 0x8F9B;
pub const GL_RGBA16: u32 = 0x805B;
pub const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const GL_CLAMP_TO_BORDER: u32 = 0x812D;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_TEXTURE_SWIZZLE_R: u32 = 36418;
pub const GL_TEXTURE_SWIZZLE_G: u32 = 36419;
pub const GL_TEXTURE_SWIZZLE_B: u32 = 36420;
pub const GL_TEXTURE_SWIZZLE_A: u32 = 36421;
pub const GL_TEXTURE_SWIZZLE_RGBA: u32 = 36422;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 36006;
pub const GL_TIME_ELAPSED: u32 = 35007;
pub const GL_QUERY_RESULT: u32 = 34918;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 34919;
pub const GL_VENDOR: u32 = 0x1F00;
pub const GL_VERSION: u32 = 0x1F02;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;

pub const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
pub const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
pub const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
pub const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
pub const WGL_ACCELERATION_ARB: u32 = 0x2003;
pub const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
pub const WGL_RED_BITS_ARB: u32 = 0x2015;
pub const WGL_RED_SHIFT_ARB: u32 = 0x2016;
pub const WGL_GREEN_BITS_ARB: u32 = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: u32 = 0x2018;
pub const WGL_BLUE_BITS_ARB: u32 = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: u32 = 0x201a;
pub const WGL_ALPHA_BITS_ARB: u32 = 0x201b;
pub const WGL_ALPHA_SHIFT_ARB: u32 = 0x201c;
pub const WGL_ACCUM_BITS_ARB: u32 = 0x201d;
pub const WGL_ACCUM_RED_BITS_ARB: u32 = 0x201e;
pub const WGL_ACCUM_GREEN_BITS_ARB: u32 = 0x201f;
pub const WGL_ACCUM_BLUE_BITS_ARB: u32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: u32 = 0x2021;
pub const WGL_DEPTH_BITS_ARB: u32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB: u32 = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: u32 = 0x2024;
pub const WGL_STEREO_ARB: u32 = 0x2012;
pub const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
pub const WGL_SAMPLES_ARB: u32 = 0x2042;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: u32 = 0x20a9;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: u32 = 0x8252;
pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: u32 = 0x8256;
pub const WGL_NO_RESET_NOTIFICATION_ARB: u32 = 0x8261;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_ARB: u32 = 0x2097;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: u32 = 0;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: u32 = 0x2098;
pub const WGL_COLORSPACE_EXT: u32 = 0x309d;
pub const WGL_COLORSPACE_SRGB_EXT: u32 = 0x3089;
pub const ERROR_INVALID_VERSION_ARB: u32 = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: u32 = 0x2096;
pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: u32 = 0x2054;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
	_unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;

static mut GL: Option<WebGl2RenderingContext> = None;

pub(crate) fn set_gl(gl: WebGl2RenderingContext) {
	unsafe {
		GL = Some(gl);
	}
}

pub(crate) fn get_gl() -> &'static WebGl2RenderingContext {
	unsafe { GL.as_ref().expect_throw("WebGL context not created!") }
}

mod counter {
	static mut COUNTER: u32 = 0;

	pub(crate) fn increment() -> u32 {
		unsafe {
			// COUNTER is always greater than zero
			COUNTER += 1;
			COUNTER
		}
	}

	#[allow(unused)]
	pub(crate) fn get() -> u32 {
		unsafe { COUNTER }
	}
}

pub fn is_gl2() -> bool {
	true
}

// ================= DATA EXTRACTION =================
pub fn glGetError() -> GLenum {
	get_gl().get_error()
}

pub fn glGetString(name: GLenum) -> *const GLubyte {
	let param = get_gl().get_parameter(name).unwrap();
	let param = param.as_string().unwrap();

	let c_str = std::ffi::CString::new(param).unwrap();
	let c_str = std::mem::ManuallyDrop::new(c_str);

	// cleaned manually for WebGL, for all invocations
	c_str.as_ptr() as _
}

// TODO: Correct implementation of glGetIntegerv
#[inline(always)]
pub(crate) unsafe fn glGetIntegerv(_: u32, data: *mut GLint) {
	#[cfg(feature = "log-impl")]
	crate::warn!("STUB: glGetIntegerv has an incomplete implementation on WebGL2");

	let data: &mut GLint = data.as_mut().unwrap();
	*data = 0;
}

// ==================== FRAME BUFFERS ====================

static mut FRAME_BUFFERS: BTreeMap<u32, WebGlFramebuffer> = BTreeMap::new();

pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
	let n = n as usize;
	let framebuffers = slice::from_raw_parts_mut(framebuffers, n);

	for i in 0..n {
		if let Some(fb) = get_gl().create_framebuffer() {
			let id = counter::increment();
			FRAME_BUFFERS.insert(id, fb);
			framebuffers[i] = id;
		}
	}
}

pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
	debug_assert!(FRAME_BUFFERS.contains_key(&framebuffer) || framebuffer == 0);
	get_gl().bind_framebuffer(target, FRAME_BUFFERS.get(&framebuffer));
}

pub unsafe fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
	debug_assert!(TEXTURES.contains_key(&texture));
	let texture = TEXTURES.get(&texture);
	get_gl().framebuffer_texture_2d(target, attachment, textarget, texture, level)
}

pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
	let framebuffers = unsafe { slice::from_raw_parts(framebuffers, n as usize) };

	for fb in framebuffers {
		let framebuffer = FRAME_BUFFERS.remove(fb);
		get_gl().delete_framebuffer(framebuffer.as_ref());
	}
}

// ================== DRAWING ==================

pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) {
	let n = n as usize;
	let bufs = slice::from_raw_parts(bufs, n);

	let array = js_sys::Array::new_with_length(bufs.len() as _);
	for (i, buf) in bufs.iter().enumerate() {
		array.set(i as u32, JsValue::from(*buf));
	}

	let gl = get_gl();
	gl.draw_buffers(&array);
}

pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
	get_gl().draw_arrays(mode, first, count);
}
pub fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const ::std::os::raw::c_void) {
	get_gl().draw_elements_with_i32(mode, count, type_, indices as i32);
}

// ==================== RENDERBUFFERS ====================

static mut RENDER_BUFFERS: BTreeMap<u32, WebGlRenderbuffer> = BTreeMap::new();

pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
	let gl = get_gl();
	let renderbuffers = slice::from_raw_parts_mut(renderbuffers, n as usize);

	for rb in renderbuffers {
		let renderbuffer = gl.create_renderbuffer().unwrap_throw();
		let idx = counter::increment();
		RENDER_BUFFERS.insert(idx, renderbuffer);
		*rb = idx as GLuint;
	}
}

pub unsafe fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
	get_gl().bind_renderbuffer(target, RENDER_BUFFERS.get(&renderbuffer));
}

pub unsafe fn glFramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffer_target: GLenum, renderbuffer: GLuint) {
	let renderbuffer = RENDER_BUFFERS.get(&renderbuffer);
	get_gl().framebuffer_renderbuffer(target, attachment, renderbuffer_target, renderbuffer);
}

pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, _params: *mut GLint) {
	get_gl().get_renderbuffer_parameter(target, pname);
	todo!("No reference implementation")
}

pub unsafe fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
	let renderbuffers = unsafe { slice::from_raw_parts(renderbuffers, n as usize) };

	for rb in renderbuffers {
		let renderbuffer = RENDER_BUFFERS.remove(rb);
		get_gl().delete_renderbuffer(renderbuffer.as_ref());
	}
}

pub unsafe fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
	let Some(_b) = RENDER_BUFFERS.get(&target) else { return 0 };
	unimplemented!("No reference implementation")
}

// ==================== VERTEX ARRAYS ====================

static mut VERTEX_ARRAY_OBJECTS: BTreeMap<u32, WebGlVertexArrayObject> = BTreeMap::new();

pub unsafe fn glGenVertexArrays(n: GLsizei, vertex_arrays: *mut GLuint) {
	let gl = get_gl();
	let arrays = slice::from_raw_parts_mut(vertex_arrays, n as usize);

	for va in arrays {
		let vao = gl.create_vertex_array().unwrap();

		let idx = counter::increment();
		VERTEX_ARRAY_OBJECTS.insert(idx, vao);

		// vertex array 0 is reserved for the null vertex array
		*va = idx as GLuint;
	}
}

pub unsafe fn glBindVertexArray(vao: GLuint) {
	debug_assert!(VERTEX_ARRAY_OBJECTS.contains_key(&vao));
	let gl = get_gl();
	gl.bind_vertex_array(VERTEX_ARRAY_OBJECTS.get(&vao));
}

// ==================== SHADERS ====================

static mut SHADERS: BTreeMap<u32, WebGlShader> = BTreeMap::new();

pub unsafe fn glCreateShader(type_: GLenum) -> GLuint {
	let gl = get_gl();
	let shader = gl.create_shader(type_).unwrap();

	// shader 0 is reserved for the null shader
	let idx = counter::increment();
	SHADERS.insert(idx, shader);
	idx
}

// _lengths is not used as pointers contains null-terminated strings, also _lengths is NULL
pub unsafe fn glShaderSource(shader_idx: GLuint, count: GLsizei, pointers: *const *const GLchar, _lengths: *const GLint) {
	debug_assert!(SHADERS.contains_key(&shader_idx));

	// get source
	let source = (0..count).into_iter().fold(String::new(), |mut acc, i| {
		let ptr = *pointers.offset(i as isize);
		let slice = std::ffi::CStr::from_ptr(ptr).to_str().unwrap();
		acc.push_str(slice);
		acc
	});

	// get shader
	let gl = get_gl();
	let shader = SHADERS.get(&shader_idx).unwrap_throw();
	gl.shader_source(shader, &source);
}

pub unsafe fn glCompileShader(shader_idx: GLuint) {
	debug_assert!(SHADERS.contains_key(&shader_idx));

	let gl = get_gl();
	let shader = SHADERS.get(&shader_idx).unwrap_throw();

	gl.compile_shader(shader);
}

static mut SHADER_LOGS: BTreeMap<u32, String> = BTreeMap::new();

pub unsafe fn glGetShaderiv(shader_idx: GLuint, pname: GLenum, params: *mut GLint) {
	debug_assert!(SHADERS.contains_key(&shader_idx));

	let gl = get_gl();
	let shader = SHADERS.get(&shader_idx).unwrap_throw();

	match pname {
		p if p == GL_INFO_LOG_LENGTH => {
			let info = gl.get_shader_info_log(shader).unwrap();
			let len = info.len() as GLint;
			SHADER_LOGS.insert(shader_idx, info);
			*params = len;
		}
		p if p == GL_SHADER_SOURCE_LENGTH => {
			let source = gl.get_shader_source(shader).unwrap();
			*params = source.len() as GLint;
		}
		p if p == GL_SHADER_TYPE => {
			let param = gl.get_shader_parameter(shader, p).as_f64().unwrap_throw() as GLint;
			*params = param;
		}
		p if p == GL_COMPILE_STATUS || p == GL_DELETE_STATUS => {
			let param = gl.get_shader_parameter(shader, p).as_bool().unwrap_throw();
			*params = param as GLint;
		}
		_ => {
			let msg = format!("glGetShaderiv failed! Invalid pname: {}", pname);
			throw_str(&msg);
		}
	}
}

pub unsafe fn glGetShaderInfoLog(shader_idx: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
	debug_assert!(SHADERS.contains_key(&shader_idx));

	// attempt to get cached log
	let mut extracted_log = None;
	let log = SHADER_LOGS.get(&shader_idx).unwrap_or_else(|| {
		let shader = SHADERS.get(&shader_idx).unwrap_throw();
		extracted_log.get_or_insert_with(|| get_gl().get_shader_info_log(shader).unwrap_throw())
	});

	let mut len = log.len() as GLsizei;

	// infoLog does not need to be nu
	if bufSize > 0 {
		len = len.min(bufSize - 1);
		let slice = log.as_bytes();
		std::ptr::copy_nonoverlapping(slice.as_ptr() as _, infoLog as _, len as usize);
	}

	*length = len;
}

// ==================== PROGRAMS & UNIFORMS==================

static mut PROGRAMS: BTreeMap<u32, WebGlProgram> = BTreeMap::new();

pub unsafe fn glCreateProgram() -> GLuint {
	let gl = get_gl();
	let program = gl.create_program().unwrap();

	// program 0 is reserved for the null program
	let idx = counter::increment();
	PROGRAMS.insert(idx, program);
	idx
}

pub unsafe fn glAttachShader(program_idx: GLuint, shader_idx: GLuint) {
	debug_assert!(SHADERS.contains_key(&shader_idx));
	debug_assert!(PROGRAMS.contains_key(&program_idx));

	let gl = get_gl();
	let shader = SHADERS.get(&shader_idx).unwrap_throw();
	let program = PROGRAMS.get(&program_idx).unwrap_throw();

	gl.attach_shader(program, shader);
}

pub unsafe fn glDetachShader(program: GLuint, shader: GLuint) {
	let shader = SHADERS.get(&shader).unwrap_throw();
	let program = PROGRAMS.get(&program).unwrap_throw();
	get_gl().detach_shader(program, shader);
}

pub unsafe fn glDeleteShader(shader: GLuint) {
	let shader = SHADERS.remove(&shader);
	get_gl().delete_shader(shader.as_ref());
}

pub unsafe fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
	let program = PROGRAMS.get(&program).unwrap_throw();
	let name = std::ffi::CStr::from_ptr(name).to_str().unwrap_throw();
	get_gl().get_attrib_location(program, name)
}
pub unsafe fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
	let program = PROGRAMS.get(&program).unwrap_throw();
	let name = std::ffi::CStr::from_ptr(name).to_str().unwrap_throw();
	get_gl().bind_attrib_location(program, index, name);
}

#[derive(Default)]
struct ProgramInfo {
	uniforms: BTreeMap<String, (WebGlActiveInfo, u32)>,
	max_uniform_length: u32,
}

static mut UNIFORMS: BTreeMap<u32, WebGlUniformLocation> = BTreeMap::new();

pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
	// If user passed an array accessor "[index]", parse the array index off the accessor.
	let mut name = std::ffi::CStr::from_ptr(name).to_str().unwrap_throw();
	let mut array_index = 0;

	// parse array index
	if name.ends_with(']') {
		let left_brace_idx = name.rfind('[').unwrap_throw();

		if left_brace_idx != name.len() - 2 {
			// input is name[..]
			array_index = name[left_brace_idx + 1..name.len() - 1].parse::<usize>().unwrap_throw();
		};
		name = &name[..left_brace_idx];
	}

	// get uniform location
	let Some(program_info) = PROGRAM_INFOS.get(&program) else { return -1 };
	let Some((info, uniform_idx)) = program_info.uniforms.get(name) else { return -1 };

	// check if array index is within bounds
	if array_index < info.size() as usize {
		return *uniform_idx as GLint + array_index as GLint;
	}

	-1 // Unable to find uniform
}

pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));
	get_gl().uniform1i(UNIFORMS.get(&(location as u32)), v0)
}

pub unsafe fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, count as usize) };
	get_gl().uniform1iv_with_i32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 2) as usize) };
	get_gl().uniform2iv_with_i32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 3) as usize) };
	get_gl().uniform3iv_with_i32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 4) as usize) };
	get_gl().uniform4iv_with_i32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, count as usize) };
	get_gl().uniform1fv_with_f32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 2) as usize) };
	get_gl().uniform2fv_with_f32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 3) as usize) };
	get_gl().uniform3fv_with_f32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 4) as usize) };
	get_gl().uniform4fv_with_f32_array(UNIFORMS.get(&(location as u32)), data);
}

pub unsafe fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
	debug_assert!(UNIFORMS.contains_key(&(location as u32)));

	let data = unsafe { slice::from_raw_parts(value, (count * 16) as usize) };
	get_gl().uniform_matrix4fv_with_f32_array(UNIFORMS.get(&(location as u32)), transpose != 0, data);
}

static mut PROGRAM_INFOS: BTreeMap<u32, ProgramInfo> = BTreeMap::new();

pub unsafe fn glLinkProgram(program_idx: GLuint) {
	debug_assert!(PROGRAMS.contains_key(&program_idx));

	let gl = get_gl();
	let program = PROGRAMS.get(&program_idx).unwrap_throw();
	gl.link_program(program);

	// setup program info
	let mut program_info = ProgramInfo::default();

	// A program's uniform table maps the string name of an uniform to an integer location of that uniform.
	// The global UNIFORMS map maps integer locations to WebGLUniformLocations.
	let uniforms = gl.get_program_parameter(program, GL_ACTIVE_UNIFORMS).as_f64().unwrap_throw() as u32;
	for i in 0..uniforms {
		let active_info = gl.get_active_uniform(program, i).unwrap_throw();
		let mut name = active_info.name();

		// This is eagerly computed below, since we already enumerate all uniforms anyway.
		program_info.max_uniform_length = (name.len() + 1) as _;

		// If we are dealing with an array, e.g. vec4 foo[3], strip off the array index part to canonicalize that "foo", "foo[]",
		// and "foo[0]" will mean the same. Loop below will populate foo[1] and foo[2].
		if name.ends_with(']') {
			let slice = name.rfind('[').unwrap_throw();
			name.truncate(slice);
		}

		// Optimize usage slightly: If we have an array of uniforms, e.g. 'vec3 colors[3];', then
		// only store the string 'colors' in utable, and 'colors[0]', 'colors[1]' and 'colors[2]' will be parsed as 'colors'+i.
		// Note that for the GL.uniforms table, we still need to fetch the all WebGLUniformLocations for all the indices.
		if let Some(loc) = gl.get_uniform_location(program, &name) {
			let uniform_id = counter::increment() as _;
			let size = active_info.size();

			program_info.uniforms.insert(name.clone(), (active_info, uniform_id));
			UNIFORMS.insert(uniform_id, loc);

			for i in 1..size {
				let name = format!("{}[{}]", name, i);
				let loc = gl.get_uniform_location(program, &name).unwrap_throw();
				UNIFORMS.insert(counter::increment(), loc);
			}
		};
	}

	// insert
	PROGRAM_INFOS.insert(program_idx, program_info);
}

pub unsafe fn glDeleteProgram(program: GLuint) {
	let program = PROGRAMS.remove(&program);
	get_gl().delete_program(program.as_ref());
}

// SAFETY: Webassembly is single-threaded
static mut PROGRAM_LOGS: BTreeMap<u32, String> = BTreeMap::new();

pub unsafe fn glGetProgramiv(program_idx: GLuint, pname: GLenum, params: *mut GLint) {
	debug_assert!(PROGRAMS.contains_key(&program_idx));

	let gl = get_gl();
	let program = PROGRAMS.get(&program_idx).unwrap_throw();

	match pname {
		p if p == GL_INFO_LOG_LENGTH => {
			let info = gl.get_program_info_log(program).unwrap();
			let len = info.len() as GLint;
			PROGRAM_LOGS.insert(program_idx, info);
			*params = len;
		}
		p if p == GL_LINK_STATUS || p == GL_DELETE_STATUS => {
			let param = gl.get_program_parameter(program, p).as_bool().unwrap_throw();
			*params = param as GLint;
		}
		_p => {
			#[cfg(feature = "log-impl")]
			crate::error!("glGetProgramiv failed! Unsupported pname: {}", _p);
		}
	}
}

pub unsafe fn glGetProgramInfoLog(program_idx: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
	debug_assert!(PROGRAMS.contains_key(&program_idx));

	// attempt to get cached log
	let mut extracted_log = None;
	let log = PROGRAM_LOGS.get(&program_idx).unwrap_or_else(|| {
		let program = PROGRAMS.get(&program_idx).unwrap_throw();
		extracted_log.get_or_insert_with(|| get_gl().get_program_info_log(program).unwrap_throw())
	});

	let mut len = log.len() as GLsizei;

	// infoLog does not need to be nu
	if bufSize > 0 {
		len = len.min(bufSize - 1);
		let slice = log.as_bytes();
		std::ptr::copy_nonoverlapping(slice.as_ptr() as _, infoLog as _, len as usize);
	}

	*length = len;
}

pub unsafe fn glUseProgram(program_idx: GLuint) {
	debug_assert!(PROGRAMS.contains_key(&program_idx));
	let gl = get_gl();
	gl.use_program(PROGRAMS.get(&program_idx));
}

// =============== INSTANCED RENDERING =================

#[inline(always)]
pub fn glVertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const ::std::os::raw::c_void) {
	get_gl().vertex_attrib_pointer_with_i32(index, size, type_, normalized != 0, stride, pointer as _);
}

#[inline(always)]
pub fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
	get_gl().vertex_attrib_divisor(index, divisor)
}

#[inline(always)]
pub fn glEnableVertexAttribArray(index: GLuint) {
	get_gl().enable_vertex_attrib_array(index)
}

#[inline(always)]
pub fn glDisableVertexAttribArray(index: GLuint) {
	get_gl().disable_vertex_attrib_array(index)
}

// ==================== TEXTURES ====================

static mut TEXTURES: BTreeMap<u32, WebGlTexture> = BTreeMap::new();

fn texture_size(internalformat: GLint, width: GLsizei, height: GLsizei) -> GLsizei {
	match internalformat as u32 {
		GL_ALPHA => width * height,
		GL_RGB => width * height * 3,
		GL_RGBA => width * height * 4,
		// TextureFormat::RGB565 | TextureFormat::RGBA4 | TextureFormat::RGBA5551
		_ => width * height * 3,
	}
}

pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
	let n = n as usize;
	let textures = slice::from_raw_parts_mut(textures, n);

	for i in 0..n {
		let gl = get_gl();
		if let Some(texture) = gl.create_texture() {
			let id = counter::increment();
			TEXTURES.insert(id, texture);
			textures[i] = id;
		}
	}
}

#[inline(always)]
pub fn glActiveTexture(texture: GLenum) {
	get_gl().active_texture(texture);
}

#[inline(always)]
pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) {
	debug_assert!(TEXTURES.contains_key(&texture) || texture == 0);
	get_gl().bind_texture(target, TEXTURES.get(&texture));
}

#[inline(always)]
pub fn glPixelStorei(pname: GLenum, param: GLint) {
	get_gl().pixel_storei(pname, param);
}

pub unsafe fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut ::std::os::raw::c_void) {
	let size = texture_size(format as _, width, height) as _;
	let pixels = (pixels as *mut u8).as_mut().map(|p| slice::from_raw_parts_mut(p, size));
	debug_assert!(pixels.as_ref().map(|p| p.len() >= size).unwrap_or(true));

	get_gl().read_pixels_with_opt_u8_array(x, y, width, height, format, type_, pixels).unwrap_throw();
}

#[inline(always)]
pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
	get_gl().tex_parameteri(target, pname, param);
}

#[inline(never)]
pub unsafe fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const ::std::os::raw::c_void) {
	let length = texture_size(internalformat, width, height) as usize;
	let pixels = (pixels as *const u8).as_ref().map(|p| slice::from_raw_parts(p, length));
	get_gl()
		.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target, level, internalformat, width, height, border, format, type_, pixels)
		.unwrap_throw();
}

#[inline(never)]
pub unsafe fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const ::std::os::raw::c_void) {
	let length = texture_size(format as _, width, height) as usize;
	let pixels = (pixels as *const u8).as_ref().map(|p| slice::from_raw_parts(p, length));
	get_gl()
		.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(target, level, xoffset, yoffset, width, height, format, type_, pixels)
		.unwrap_throw();
}

#[inline(always)]
pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
	get_gl().copy_tex_image_2d(target, level, internalformat, x, y, width, height, border);
}

#[inline(always)]
pub fn glCopyTexSubImage2D(target: GLenum, level: GLint, x_offset: GLint, y_offset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
	get_gl().copy_tex_sub_image_2d(target, level, x_offset, y_offset, x, y, width, height);
}

pub unsafe fn glCompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const ::std::os::raw::c_void) {
	match data.as_ref().map(|_| std::slice::from_raw_parts(data as *const u8, imageSize as usize)) {
		Some(data) => get_gl().compressed_tex_image_2d_with_u8_array(target, level, internalformat, width, height, border, data),
		None => {
			get_gl().compressed_tex_image_2d_with_i32_and_i32(target, level, internalformat, width, height, border, imageSize, 0);
		}
	};
}

pub unsafe fn glCompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const ::std::os::raw::c_void) {
	let length = imageSize as usize;
	let data = std::slice::from_raw_parts_mut(data as *mut u8, length);

	get_gl().compressed_tex_sub_image_2d_with_u8_array(target, level, xoffset, yoffset, width, height, format, data);
}

pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
	let n = n as usize;
	let textures = slice::from_raw_parts(textures, n);

	for t in textures {
		let texture = TEXTURES.get(t);
		get_gl().delete_texture(texture);
	}
}

#[inline(always)]
pub fn glGenerateMipmap(target: GLenum) {
	get_gl().generate_mipmap(target);
}

// ==================== BLENDING ====================

#[inline(always)]
pub fn glEnable(cap: GLenum) {
	get_gl().enable(cap)
}

#[inline(always)]
pub fn glDisable(cap: GLenum) {
	get_gl().disable(cap)
}

#[inline(always)]
pub fn glBlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
	get_gl().blend_func_separate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
}

#[inline(always)]
pub fn glBlendEquationSeparate(mode_rgb: GLenum, mode_alpha: GLenum) {
	get_gl().blend_equation_separate(mode_rgb, mode_alpha)
}

#[inline(always)]
pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
	get_gl().blend_func(sfactor, dfactor)
}

#[inline(always)]
pub fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
	get_gl().blend_color(red, green, blue, alpha)
}

#[inline(always)]
pub fn glBlendEquation(mode: GLenum) {
	get_gl().blend_equation(mode)
}

#[inline(always)]
pub fn glDepthFunc(func: GLenum) {
	get_gl().depth_func(func)
}

#[inline(always)]
pub fn glCullFace(mode: GLenum) {
	get_gl().cull_face(mode)
}

#[inline(always)]
pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
	get_gl().color_mask(red != 0, green != 0, blue != 0, alpha != 0)
}

#[inline(always)]
pub fn glFrontFace(mode: GLenum) {
	get_gl().front_face(mode)
}

#[inline(always)]
pub fn glDepthMask(flag: GLboolean) {
	get_gl().depth_mask(flag != 0)
}

#[inline(always)]
pub fn glDepthRangef(n: GLfloat, f: GLfloat) {
	get_gl().depth_range(n, f)
}

// ==================== STENCILS & CULLING====================

#[inline(always)]
pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint) {
	get_gl().stencil_func(func, ref_, mask)
}

#[inline(always)]
pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
	get_gl().stencil_func_separate(face, func, ref_, mask)
}

#[inline(always)]
pub fn glStencilMask(mask: GLuint) {
	get_gl().stencil_mask(mask)
}

#[inline(always)]
pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
	get_gl().stencil_mask_separate(face, mask)
}

#[inline(always)]
pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
	get_gl().stencil_op(fail, zfail, zpass)
}

#[inline(always)]
pub fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
	get_gl().stencil_op_separate(face, sfail, dpfail, dppass)
}

// ============= GPU QUERIES ================
static mut QUERIES: BTreeMap<u32, WebGlQuery> = BTreeMap::new();

pub unsafe fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
	debug_assert!(ids.as_ref().is_some());

	let n = n as usize;
	let ids = slice::from_raw_parts_mut(ids, n);

	for id in ids {
		if let Some(query) = get_gl().create_query() {
			*id = counter::increment();
			QUERIES.insert(*id, query);
		}
	}
}

#[inline(always)]
pub unsafe fn glBeginQuery(target: GLenum, id: GLuint) {
	debug_assert!(QUERIES.contains_key(&id));

	if let Some(query) = QUERIES.get(&id) {
		get_gl().begin_query(target, query);
	}
}

#[inline(always)]
pub fn glEndQuery(target: GLenum) {
	get_gl().end_query(target);
}

#[inline(always)]
pub unsafe fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
	debug_assert!(ids.as_ref().is_some());

	for id in slice::from_raw_parts(ids, n as usize) {
		let query = QUERIES.remove(id);
		get_gl().delete_query(query.as_ref());
	}
}

// ============= BUFFERS ================
static mut BUFFERS: BTreeMap<u32, WebGlBuffer> = BTreeMap::new();

pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
	let n = n as usize;
	debug_assert!(buffers.as_ref().is_some());

	for id in slice::from_raw_parts_mut(buffers, n) {
		if let Some(buffer) = get_gl().create_buffer() {
			*id = counter::increment();
			BUFFERS.insert(*id, buffer);
		}
	}
}

#[inline(always)]
pub unsafe fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const ::std::os::raw::c_void, usage: GLenum) {
	if data.is_null() {
		get_gl().buffer_data_with_i32(target, size, usage);
	} else {
		let data = slice::from_raw_parts(data as *const u8, size as usize);
		get_gl().buffer_data_with_u8_array(target, data, usage);
	}
}

pub unsafe fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const ::std::os::raw::c_void) {
	let data = slice::from_raw_parts(data as *const u8, size as usize);
	get_gl().buffer_sub_data_with_i32_and_u8_array(target, offset, data);
}

#[inline(always)]
pub unsafe fn glBindBuffer(target: GLenum, buffer: GLuint) {
	debug_assert!(BUFFERS.contains_key(&buffer) || buffer == 0);
	get_gl().bind_buffer(target, BUFFERS.get(&buffer));
}

pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
	let n = n as usize;
	debug_assert!(buffers.as_ref().is_some());

	for id in slice::from_raw_parts(buffers, n) {
		let buffer = BUFFERS.remove(id);
		get_gl().delete_buffer(buffer.as_ref());
	}
}

// ================ VIEWPORT ====================

#[inline(always)]
pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
	get_gl().viewport(x, y, width, height);
}

#[inline(always)]
pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
	get_gl().scissor(x, y, width, height);
}

// =============== INSTANCED RENDERING ========
#[inline(always)]
pub fn glDrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instance_count: GLsizei) {
	get_gl().draw_arrays_instanced(mode, first, count, instance_count)
}

#[inline(always)]
pub fn glDrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const ::std::os::raw::c_void, instance_count: GLsizei) {
	get_gl().draw_elements_instanced_with_i32(mode, count, type_, indices as _, instance_count)
}

// ================ CLEAR COLOUR ========

#[inline(always)]
pub fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
	get_gl().clear_color(red, green, blue, alpha)
}

#[inline(always)]
pub fn glClearDepthf(depth: GLfloat) {
	get_gl().clear_depth(depth)
}

#[inline(always)]
pub fn glClear(mask: GLbitfield) {
	get_gl().clear(mask)
}

#[inline(always)]
pub fn glClearStencil(s: GLint) {
	get_gl().clear_stencil(s)
}
// ================ * ====================
#[inline(always)]
pub fn glFinish() {
	get_gl().finish();
}

#[inline(always)]
pub fn glFlush() {
	get_gl().flush();
}

// extern "C" {
// 	pub fn glGetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
// 	pub fn glGetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
// 	pub fn glGetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
// 	pub fn glGetBooleanv(pname: GLenum, data: *mut GLboolean);
// 	pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
// 	pub fn glGetFloatv(pname: GLenum, data: *mut GLfloat);
// 	pub fn glGetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
// 	pub fn glGetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
// 	pub fn glGetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
// 	pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
// 	pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
// 	pub fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
// 	pub fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
// 	pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
// 	pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut ::std::os::raw::c_void);
// 	pub fn glHint(target: GLenum, mode: GLenum);
// 	pub fn glIsBuffer(buffer: GLuint) -> GLboolean;
// 	pub fn glIsEnabled(cap: GLenum) -> GLboolean;
// 	pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;
// 	pub fn glIsProgram(program: GLuint) -> GLboolean;
// 	pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
// 	pub fn glIsShader(shader: GLuint) -> GLboolean;
// 	pub fn glIsTexture(texture: GLuint) -> GLboolean;
// 	pub fn glLineWidth(width: GLfloat);
// 	pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
// 	pub fn glReleaseShaderCompiler();
// 	pub fn glRenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
// 	pub fn glSampleCoverage(value: GLfloat, invert: GLboolean);
// 	pub fn glShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const ::std::os::raw::c_void, length: GLsizei);

// 	pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
// 	pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
// 	pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
// 	pub fn glUniform1f(location: GLint, v0: GLfloat);
// 	pub fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
// 	pub fn glUniform2i(location: GLint, v0: GLint, v1: GLint);
// 	pub fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
// 	pub fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
// 	pub fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
// 	pub fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
// 	pub fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glValidateProgram(program: GLuint);
// 	pub fn glVertexAttrib1f(index: GLuint, x: GLfloat);
// 	pub fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat);
// 	pub fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
// 	pub fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat);
// 	pub fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
// 	pub fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat);
// 	pub fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
// 	pub fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat);
// 	pub fn glReadBuffer(src: GLenum);
// 	pub fn glDrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const ::std::os::raw::c_void);
// 	pub fn glTexImage3D(
// 		target: GLenum,
// 		level: GLint,
// 		internalformat: GLint,
// 		width: GLsizei,
// 		height: GLsizei,
// 		depth: GLsizei,
// 		border: GLint,
// 		format: GLenum,
// 		type_: GLenum,
// 		pixels: *const ::std::os::raw::c_void,
// 	);
// 	pub fn glTexSubImage3D(
// 		target: GLenum,
// 		level: GLint,
// 		xoffset: GLint,
// 		yoffset: GLint,
// 		zoffset: GLint,
// 		width: GLsizei,
// 		height: GLsizei,
// 		depth: GLsizei,
// 		format: GLenum,
// 		type_: GLenum,
// 		pixels: *const ::std::os::raw::c_void,
// 	);
// 	pub fn glCopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
// 	pub fn glCompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const ::std::os::raw::c_void);
// 	pub fn glCompressedTexSubImage3D(
// 		target: GLenum,
// 		level: GLint,
// 		xoffset: GLint,
// 		yoffset: GLint,
// 		zoffset: GLint,
// 		width: GLsizei,
// 		height: GLsizei,
// 		depth: GLsizei,
// 		format: GLenum,
// 		imageSize: GLsizei,
// 		data: *const ::std::os::raw::c_void,
// 	);

// 	pub fn glIsQuery(id: GLuint) -> GLboolean;
// 	pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);
// 	pub fn glQueryCounter(id: GLenum, pname: GLenum);
// 	pub fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64);

// 	pub fn glUnmapBuffer(target: GLenum) -> GLboolean;
// 	pub fn glGetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut ::std::os::raw::c_void);
// 	pub fn glUniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glUniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 	pub fn glBlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
// 	pub fn glRenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
// 	pub fn glFramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
// 	pub fn glMapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut ::std::os::raw::c_void;
// 	pub fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);
// 	pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
// 	pub fn glIsVertexArray(array: GLuint) -> GLboolean;
// 	pub fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);
// 	pub fn glBeginTransformFeedback(primitiveMode: GLenum);
// 	pub fn glEndTransformFeedback();
// 	pub fn glBindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
// 	pub fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);
// 	pub fn glTransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
// 	pub fn glGetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
// 	pub fn glVertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const ::std::os::raw::c_void);
// 	pub fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);
// 	pub fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
// 	pub fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
// 	pub fn glVertexAttribI4iv(index: GLuint, v: *const GLint);
// 	pub fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint);
// 	pub fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
// 	pub fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
// 	pub fn glUniform1ui(location: GLint, v0: GLuint);
// 	pub fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint);
// 	pub fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
// 	pub fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
// 	pub fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
// 	pub fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
// 	pub fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
// 	pub fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
// 	pub fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
// 	pub fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
// 	pub fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
// 	pub fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
// 	pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
// 	pub fn glCopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
// 	pub fn glGetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
// 	pub fn glGetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
// 	pub fn glGetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
// 	pub fn glUniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

// 	pub fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync;
// 	pub fn glIsSync(sync: GLsync) -> GLboolean;
// 	pub fn glDeleteSync(sync: GLsync);
// 	pub fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
// 	pub fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
// 	pub fn glGetInteger64v(pname: GLenum, data: *mut GLint64);
// 	pub fn glGetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint);
// 	pub fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);
// 	pub fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);
// 	pub fn glGenSamplers(count: GLsizei, samplers: *mut GLuint);
// 	pub fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint);
// 	pub fn glIsSampler(sampler: GLuint) -> GLboolean;
// 	pub fn glBindSampler(unit: GLuint, sampler: GLuint);
// 	pub fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);
// 	pub fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);
// 	pub fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);
// 	pub fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);
// 	pub fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);
// 	pub fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
// 	pub fn glBindTransformFeedback(target: GLenum, id: GLuint);
// 	pub fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);
// 	pub fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
// 	pub fn glIsTransformFeedback(id: GLuint) -> GLboolean;
// 	pub fn glPauseTransformFeedback();
// 	pub fn glResumeTransformFeedback();
// 	pub fn glGetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut ::std::os::raw::c_void);
// 	pub fn glProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const ::std::os::raw::c_void, length: GLsizei);
// 	pub fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint);
// 	pub fn glInvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
// 	pub fn glInvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
// 	pub fn glTexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
// 	pub fn glTexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
// 	pub fn glGetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint);
// }
