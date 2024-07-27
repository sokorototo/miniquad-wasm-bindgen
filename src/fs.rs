/// A file-system loading error.
///
/// On `cfg!(target_arch = "wasm32")` contains a 5th variant `DownloadFailed(String)` containing the response status text.
#[derive(Debug)]
pub enum Error {
	IOError(std::io::Error),
	/// XmlHttpRequest failed, returns [`ProgressEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent) and Status Text
	#[cfg(target_arch = "wasm32")]
	DownloadFailed(String),
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
/// Check for `DownloadFailed(String)` on WASM
pub fn load_file<F: FnOnce(Response) + 'static>(path: &str, on_loaded: F) {
	#[cfg(target_arch = "wasm32")]
	wasm::load_file(path, on_loaded);

	#[cfg(not(target_arch = "wasm32"))]
	load_file_desktop(path, on_loaded);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
	use super::{Error, Response};

	use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
	use wasm_bindgen_futures::*;
	use web_sys::*;

	pub fn load_file<F: FnOnce(Response) + 'static>(path: &str, on_loaded: F) {
		let window = window().unwrap();

		// create abort signal
		let signal = || {
			let controller = AbortController::new().ok()?;
			let signal = controller.signal();

			let closure: Closure<dyn Fn()> = Closure::new(move || controller.abort());
			let js_callback = closure.into_js_value();

			window.set_timeout_with_callback_and_timeout_and_arguments_0(js_callback.dyn_ref()?, 5 * 1000).ok()?;

			Some(signal)
		};

		let mut opts = RequestInit::new();
		opts.method("GET");
		opts.mode(RequestMode::Cors);
		opts.signal(signal().as_ref());

		if let Ok(req) = Request::new_with_str_and_init(path, &opts) {
			let promise = window.fetch_with_request(&req);
			let future = JsFuture::from(promise);

			spawn_local(async move {
				match future.await {
					Ok(res) => {
						let res: web_sys::Response = res.dyn_into().unwrap();

						match !res.ok() {
							true => {
								let status_text = res.status_text();
								#[cfg(feature = "log-impl")]
								crate::error!("fetch failed: {:?}", &status_text);
								on_loaded(Err(Error::DownloadFailed(status_text)));
							}
							false => {
								let ab = res.array_buffer().unwrap_throw();
								match JsFuture::from(ab).await {
									Ok(ab) => {
										let array = js_sys::Uint8Array::new(&ab).to_vec();
										return on_loaded(Ok(array));
									}
									Err(err) => {
										let msg = "Unable to extract data from array buffer";

										#[cfg(feature = "log-impl")]
										console::error_2(&JsValue::from_str(msg), &err);

										on_loaded(Err(Error::DownloadFailed(msg.into())));
									}
								}
							}
						}
					}
					Err(err) => {
						let msg = "Unable to call window.fetch, check console for error logs";

						#[cfg(feature = "log-impl")]
						console::error_2(&JsValue::from_str(msg), &err);

						on_loaded(Err(Error::DownloadFailed(msg.into())));
					}
				}
			});
		}
	}
}

#[cfg(not(target_arch = "wasm32"))]
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
