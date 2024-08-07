/// This module is disabled by default
///
/// Most of the code gleaned from log-rs crate
///
/// Will send log calls like debug!(), warn!() and error!() to appropriate console_* call on wasm
/// and just println! on PC.
/// If you need better control of log messages - just dont use "log-impl" feature and use appropriate loggers from log-rs
use std::cmp;

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Level {
	/// The "error" level.
	///
	/// Designates very serious errors.
	Error = 1, // This way these line up with the discriminants for LevelFilter below
	/// The "warn" level.
	///
	/// Designates hazardous situations.
	Warn,
	/// The "info" level.
	///
	/// Designates useful information.
	Info,
	/// The "debug" level.
	///
	/// Designates lower priority information.
	Debug,
	/// The "trace" level.
	///
	/// Designates very low priority, often extremely verbose, information.
	Trace,
}

impl PartialOrd for Level {
	#[inline]
	fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}

	#[inline]
	fn lt(&self, other: &Level) -> bool {
		(*self as usize) < *other as usize
	}

	#[inline]
	fn le(&self, other: &Level) -> bool {
		*self as usize <= *other as usize
	}

	#[inline]
	fn gt(&self, other: &Level) -> bool {
		*self as usize > *other as usize
	}

	#[inline]
	fn ge(&self, other: &Level) -> bool {
		*self as usize >= *other as usize
	}
}

impl Ord for Level {
	#[inline]
	fn cmp(&self, other: &Level) -> cmp::Ordering {
		(*self as usize).cmp(&(*other as usize))
	}
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $message:expr) => ({
        let lvl = $lvl;
        //if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            // ensure that $message is a valid format string literal
            let _ = __log_format_args!($message);
            $crate::log::__private_api_log_lit(
                $message,
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
            );
        //}
    });
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        //if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::log::__private_api_log_lit(
                &__log_format_args!($($arg)+),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
            );
        //}
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Error, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Warn, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Info, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Debug, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Trace, $($arg)+);
    )
}

/// log-rs used `macro_export(local_inner_macros)` instead of $crate::log! to support older rustc version
/// but actually there is an other difference - $crate::log does not support macros reexport :(
/// so even miniquad_wasm_bindgen is fine with 1.31+ rustc version, we need to use local_inner_macros as well
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
	() => {
		module_path!()
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
	() => {
		file!()
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
	() => {
		line!()
	};
}

#[cfg(not(target_arch = "wasm32"))]
pub fn __private_api_log_lit(message: &str, level: Level, &(_target, _module_path, _file, _line): &(&str, &'static str, &'static str, u32)) {
	let prefix = match level {
		Level::Error => "ERROR",
		Level::Warn => "WARN",
		Level::Info => "INFO",
		Level::Debug => "DEBUG",
		Level::Trace => "TRACE",
	};

	eprintln!("[{}]({}:{}:0) {}", prefix, _file, _line, message);
}

#[cfg(target_arch = "wasm32")]
pub fn __private_api_log_lit(message: &str, level: Level, &(_, _, file, line): &(&str, &'static str, &'static str, u32)) {
	use web_sys::console;

	match level {
		Level::Debug => {
			let header = "%c[DEBUG]".into();
			let style = "color: blue; font-weight: bold".into();
			let message = format!("{}:{}\n{}", file, line, message).into();

			console::debug_3(&header, &style, &message);
		}
		Level::Warn => {
			let header = "%c[WARN]".into();
			let style = "color: orange; font-weight: bold".into();
			let message = format!("{}:{}\n{}", file, line, message).into();

			console::warn_3(&header, &style, &message);
		}
		Level::Info => {
			let header = "%c[INFO]".into();
			let style = "color: green; font-weight: bold".into();
			let message = format!("{}:{}\n{}", file, line, message).into();

			console::info_3(&header, &style, &message);
		}
		Level::Trace => {
			let header = "%c[TRACE]".into();
			let style = "color: grey; font-weight: bold".into();
			let message = format!("{}:{}\n{}", file, line, message).into();

			console::log_3(&header, &style, &message);
		}
		Level::Error => {
			let header = "%c[ERROR]".into();
			let style = "color: red; font-weight: bold".into();
			let message = format!("{}:{}\n{}", file, line, message).into();

			console::error_3(&header, &style, &message);
		}
	}
}
