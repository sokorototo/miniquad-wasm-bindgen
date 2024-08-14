use miniquad_wasm_bindgen::*;

#[derive(Debug, Default)]
struct State {
	updates: u128,
	draws: u128,
}

impl EventHandler for State {
	fn update(&mut self) {
		self.updates += 1;

		#[cfg(feature = "log-impl")]
		info!("Update Cycles: {:?}", self.updates)
	}

	fn draw(&mut self) {
		self.draws += 1;

		#[cfg(feature = "log-impl")]
		info!("Draw Cycles: {:?}", self.draws)
	}

	fn mouse_button_down_event(&mut self, button: MouseButton, _: f32, _: f32) {
		if let MouseButton::Left = button {
			window::schedule_update();
		}
	}

	fn files_dropped_event(&mut self, paths: Vec<std::path::PathBuf>, _: Option<Vec<Vec<u8>>>) {
		#[cfg(feature = "log-impl")]
		for path in paths {
			info!("File Dropped into App: {:?}", path)
		}
	}
}

fn main() {
	let mut conf = conf::Conf::default();
	conf.platform = conf::PlatformSettings {
		blocking_event_loop: true,
		..Default::default()
	};

	miniquad_wasm_bindgen::start(conf, || Box::new(State::default()));
}
