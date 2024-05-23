use crate::event::KeyCode;

pub fn translate_keycode(keycode: i32) -> KeyCode {
	match keycode {
		32 => KeyCode::Space,
		39 => KeyCode::Apostrophe,
		44 => KeyCode::Comma,
		45 => KeyCode::Minus,
		46 => KeyCode::Period,
		47 => KeyCode::Slash,
		48 => KeyCode::Key0,
		49 => KeyCode::Key1,
		50 => KeyCode::Key2,
		51 => KeyCode::Key3,
		52 => KeyCode::Key4,
		53 => KeyCode::Key5,
		54 => KeyCode::Key6,
		55 => KeyCode::Key7,
		56 => KeyCode::Key8,
		57 => KeyCode::Key9,
		59 => KeyCode::Semicolon,
		61 => KeyCode::Equal,
		65 => KeyCode::A,
		66 => KeyCode::B,
		67 => KeyCode::C,
		68 => KeyCode::D,
		69 => KeyCode::E,
		70 => KeyCode::F,
		71 => KeyCode::G,
		72 => KeyCode::H,
		73 => KeyCode::I,
		74 => KeyCode::J,
		75 => KeyCode::K,
		76 => KeyCode::L,
		77 => KeyCode::M,
		78 => KeyCode::N,
		79 => KeyCode::O,
		80 => KeyCode::P,
		81 => KeyCode::Q,
		82 => KeyCode::R,
		83 => KeyCode::S,
		84 => KeyCode::T,
		85 => KeyCode::U,
		86 => KeyCode::V,
		87 => KeyCode::W,
		88 => KeyCode::X,
		89 => KeyCode::Y,
		90 => KeyCode::Z,
		91 => KeyCode::LeftBracket,
		92 => KeyCode::Backslash,
		93 => KeyCode::RightBracket,
		96 => KeyCode::Apostrophe,
		256 => KeyCode::Escape,
		257 => KeyCode::Enter,
		258 => KeyCode::Tab,
		259 => KeyCode::Backspace,
		260 => KeyCode::Insert,
		261 => KeyCode::Delete,
		262 => KeyCode::Right,
		263 => KeyCode::Left,
		264 => KeyCode::Down,
		265 => KeyCode::Up,
		266 => KeyCode::PageUp,
		267 => KeyCode::PageDown,
		268 => KeyCode::Home,
		269 => KeyCode::End,
		280 => KeyCode::CapsLock,
		281 => KeyCode::ScrollLock,
		282 => KeyCode::NumLock,
		283 => KeyCode::PrintScreen,
		284 => KeyCode::Pause,
		290 => KeyCode::F1,
		291 => KeyCode::F2,
		292 => KeyCode::F3,
		293 => KeyCode::F4,
		294 => KeyCode::F5,
		295 => KeyCode::F6,
		296 => KeyCode::F7,
		297 => KeyCode::F8,
		298 => KeyCode::F9,
		299 => KeyCode::F10,
		300 => KeyCode::F11,
		301 => KeyCode::F12,
		302 => KeyCode::F13,
		303 => KeyCode::F14,
		304 => KeyCode::F15,
		305 => KeyCode::F16,
		306 => KeyCode::F17,
		307 => KeyCode::F18,
		308 => KeyCode::F19,
		309 => KeyCode::F20,
		310 => KeyCode::F21,
		311 => KeyCode::F22,
		312 => KeyCode::F23,
		313 => KeyCode::F24,
		320 => KeyCode::Kp0,
		321 => KeyCode::Kp1,
		322 => KeyCode::Kp2,
		323 => KeyCode::Kp3,
		324 => KeyCode::Kp4,
		325 => KeyCode::Kp5,
		326 => KeyCode::Kp6,
		327 => KeyCode::Kp7,
		328 => KeyCode::Kp8,
		329 => KeyCode::Kp9,
		330 => KeyCode::KpDecimal,
		331 => KeyCode::KpDivide,
		332 => KeyCode::KpMultiply,
		333 => KeyCode::KpSubtract,
		334 => KeyCode::KpAdd,
		335 => KeyCode::KpEnter,
		336 => KeyCode::KpEqual,
		340 => KeyCode::LeftShift,
		341 => KeyCode::LeftControl,
		342 => KeyCode::LeftAlt,
		343 => KeyCode::LeftSuper,
		344 => KeyCode::RightShift,
		345 => KeyCode::RightControl,
		346 => KeyCode::RightAlt,
		347 => KeyCode::RightSuper,
		348 => KeyCode::Menu,
		_ => KeyCode::Unknown,
	}
}

pub fn get_keycode(key: &str) -> Option<i32> {
	Some(match key {
		"Space" => 32,
		"Quote" => 222,
		"Comma" => 44,
		"Minus" => 45,
		"Period" => 46,
		"Slash" => 189,
		"Digit0" => 48,
		"Digit1" => 49,
		"Digit2" => 50,
		"Digit3" => 51,
		"Digit4" => 52,
		"Digit5" => 53,
		"Digit6" => 54,
		"Digit7" => 55,
		"Digit8" => 56,
		"Digit9" => 57,
		"Semicolon" => 59,
		"Equal" => 61,
		"KeyA" => 65,
		"KeyB" => 66,
		"KeyC" => 67,
		"KeyD" => 68,
		"KeyE" => 69,
		"KeyF" => 70,
		"KeyG" => 71,
		"KeyH" => 72,
		"KeyI" => 73,
		"KeyJ" => 74,
		"KeyK" => 75,
		"KeyL" => 76,
		"KeyM" => 77,
		"KeyN" => 78,
		"KeyO" => 79,
		"KeyP" => 80,
		"KeyQ" => 81,
		"KeyR" => 82,
		"KeyS" => 83,
		"KeyT" => 84,
		"KeyU" => 85,
		"KeyV" => 86,
		"KeyW" => 87,
		"KeyX" => 88,
		"KeyY" => 89,
		"KeyZ" => 90,
		"BracketLeft" => 91,
		"Backslash" => 92,
		"BracketRight" => 93,
		"Backquote" => 96,
		"Escape" => 256,
		"Enter" => 257,
		"Tab" => 258,
		"Backspace" => 259,
		"Insert" => 260,
		"Delete" => 261,
		"ArrowRight" => 262,
		"ArrowLeft" => 263,
		"ArrowDown" => 264,
		"ArrowUp" => 265,
		"PageUp" => 266,
		"PageDown" => 267,
		"Home" => 268,
		"End" => 269,
		"CapsLock" => 280,
		"ScrollLock" => 281,
		"NumLock" => 282,
		"PrintScreen" => 283,
		"Pause" => 284,
		"F1" => 290,
		"F2" => 291,
		"F3" => 292,
		"F4" => 293,
		"F5" => 294,
		"F6" => 295,
		"F7" => 296,
		"F8" => 297,
		"F9" => 298,
		"F10" => 299,
		"F11" => 300,
		"F12" => 301,
		"F13" => 302,
		"F14" => 303,
		"F15" => 304,
		"F16" => 305,
		"F17" => 306,
		"F18" => 307,
		"F19" => 308,
		"F20" => 309,
		"F21" => 310,
		"F22" => 311,
		"F23" => 312,
		"F24" => 313,
		"Numpad0" => 320,
		"Numpad1" => 321,
		"Numpad2" => 322,
		"Numpad3" => 323,
		"Numpad4" => 324,
		"Numpad5" => 325,
		"Numpad6" => 326,
		"Numpad7" => 327,
		"Numpad8" => 328,
		"Numpad9" => 329,
		"NumpadDecimal" => 330,
		"NumpadDivide" => 331,
		"NumpadMultiply" => 33,
		"NumpadSubtract" => 33,
		"NumpadAdd" => 334,
		"NumpadEnter" => 335,
		"NumpadEqual" => 336,
		"ShiftLeft" => 340,
		"ControlLeft" => 341,
		"AltLeft" => 342,
		"OSLeft" => 343,
		"ShiftRight" => 344,
		"ControlRight" => 345,
		"AltRight" => 346,
		"OSRight" => 347,
		"ContextMenu" => 348,
		_ => return None,
	})
}
