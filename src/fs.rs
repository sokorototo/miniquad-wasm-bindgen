#[cfg(target_os = "android")]
use crate::native::android;
#[cfg(target_os = "ios")]
use crate::native::ios;

/// A file-system loading error.
///
/// On `cfg!(target_arch = "wasm32")` contains a 5th variant `DownloadFailed((web_sys::ProgressEvent, Option<String>))` that stores download errors.
#[derive(Debug)]
pub enum Error {
	IOError(std::io::Error),
	/// XmlHttpRequest failed, returns [`ProgressEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent) and Status Text
	#[cfg(target_arch = "wasm32")]
	DownloadFailed((web_sys::ProgressEvent, Option<String>)),
	AndroidAssetLoadingError,
	/// MainBundle pathForResource returned null
	IOSAssetNoSuchFile,
	/// NSData dataWithContentsOfFile or data.bytes are null
	IOSAssetNoData,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Error: {:?}", self)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::IOError(e)
	}
}

pub type Response = Result<Vec<u8>, Error>;

/// Filesystem path on desktops or HTTP URL in WASM.
///
/// Check for `DownloadFailed((web_sys::ProgressEvent, Option<String>))` on WASM
pub fn load_file<F: FnOnce(Response) + 'static>(path: &str, on_loaded: F) {
	#[cfg(target_arch = "wasm32")]
	wasm::load_file(path, on_loaded);

	#[cfg(target_os = "android")]
	load_file_android(path, on_loaded);

	#[cfg(target_os = "ios")]
	ios::load_file(path, on_loaded);

	#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
	load_file_desktop(path, on_loaded);
}

#[cfg(target_os = "android")]
fn load_file_android<F: FnOnce(Response)>(path: &str, on_loaded: F) {
	fn load_file_sync(path: &str) -> Response {
		let filename = std::ffi::CString::new(path).unwrap();
		match android::load_asset(&filename) {
			Some(data) => Ok(data),
			None => Err(Error::AndroidAssetLoadingError),
		}
	}

	let response = load_file_sync(path);
	on_loaded(response);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
	use super::{Error, Response};

	use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
	use web_sys::*;

	pub fn load_file<F: FnOnce(Response) + 'static>(path: &str, on_loaded: F) {
		if let Ok(xhr) = XmlHttpRequest::new() {
			if xhr.open("GET", path).is_ok() {
				xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
				xhr.set_timeout(5 * 1000); // 5 seconds

				let xhr_1 = xhr.clone();
				let present = Closure::once_into_js(move |ev: ProgressEvent| {
					match xhr_1.response() {
						Ok(d) => {
							if xhr_1.status().unwrap() != 200 {
								#[cfg(feature = "log-impl")]
								crate::error!("XmlHttpRequest failed: {:?}", xhr_1.status_text());
								on_loaded(Err(Error::DownloadFailed((ev, xhr_1.status_text().ok()))));
							} else {
								let array = d.dyn_into::<js_sys::ArrayBuffer>().unwrap_throw();
								let array = js_sys::Uint8Array::new(&array).to_vec();
								on_loaded(Ok(array));
							}
						}
						Err(_) => on_loaded(Err(Error::DownloadFailed((ev, xhr_1.status_text().ok())))),
					};
				});

				xhr.set_ontimeout(Some(present.as_ref().unchecked_ref()));
				xhr.set_onerror(Some(present.as_ref().unchecked_ref()));
				xhr.set_onload(Some(present.as_ref().unchecked_ref()));
			} else {
				let err = (ProgressEvent::new("Unable to Open XmlHttpRequest").unwrap(), None);
				on_loaded(Err(Error::DownloadFailed(err)));
			};

			// Send the request
			xhr.send().unwrap();
		}
	}
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
fn load_file_desktop<F: FnOnce(Response)>(path: &str, on_loaded: F) {
	fn load_file_sync(path: &str) -> Response {
		use std::fs::File;
		use std::io::Read;

		let mut response = vec![];
		let mut file = File::open(path)?;
		file.read_to_end(&mut response)?;
		Ok(response)
	}

	let response = load_file_sync(path);

	on_loaded(response);
}
