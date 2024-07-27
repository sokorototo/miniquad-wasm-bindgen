use conf::LinuxBackend;
use miniquad_wasm_bindgen::*;

#[repr(C)]
struct Vec2 {
	x: f32,
	y: f32,
}

#[repr(C)]
struct Vertex {
	pos: Vec2,
	uv: Vec2,
}

struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
	start_time: f64,
	then: f64,
	uniforms: shader::Uniforms,
	blobs_velocities: [(f32, f32); 32],
	ctx: Box<dyn RenderingBackend>,
}

impl Stage {
	pub fn new() -> Stage {
		let mut ctx = window::new_rendering_backend();

		#[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
		let vertex_buffer = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));

		let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
		let index_buffer = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer,
			images: vec![],
		};

		let shader = ctx.new_shader(ShaderSource::new(shader::VERTEX, shader::FRAGMENT), shader::meta()).unwrap();

		let pipeline = ctx.new_pipeline(
			&[BufferLayout::default()],
			&[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_uv", VertexFormat::Float2)],
			shader,
			PipelineParams::default(),
		);

		let uniforms = shader::Uniforms {
			time: 0.,
			blobs_count: 1,
			blobs_positions: [(0., 0.); 32],
		};

		let time = miniquad_wasm_bindgen::date::now();

		Stage {
			pipeline,
			bindings,
			start_time: time,
			uniforms,
			blobs_velocities: [(0., 0.); 32],
			then: time,
			ctx,
		}
	}
}

impl EventHandler for Stage {
	fn update(&mut self) {
		let now = miniquad_wasm_bindgen::date::now();
		let delta = (now - self.then) as f32;

		for i in 1..self.uniforms.blobs_count as usize {
			self.uniforms.blobs_positions[i].0 += self.blobs_velocities[i].0 * delta * 0.1;
			self.uniforms.blobs_positions[i].1 += self.blobs_velocities[i].1 * delta * 0.1;

			if self.uniforms.blobs_positions[i].0 < 0. || self.uniforms.blobs_positions[i].0 > 1. {
				self.blobs_velocities[i].0 *= -1.;
			}
			if self.uniforms.blobs_positions[i].1 < 0. || self.uniforms.blobs_positions[i].1 > 1. {
				self.blobs_velocities[i].1 *= -1.;
			}
		}

		self.then = now;
	}

	fn mouse_motion_event(&mut self, x: f32, y: f32) {
		let (w, h) = {
			let (w, h) = window::screen_size();
			(w as f32, h as f32)
		};
		let (x, y) = (x / w, 1. - y / h);
		self.uniforms.blobs_positions[0] = (x, y);
	}

	fn mouse_button_down_event(&mut self, _button: MouseButton, x: f32, y: f32) {
		if self.uniforms.blobs_count >= 32 {
			return;
		}

		let (w, h) = {
			let (w, h) = window::screen_size();
			(w as f32, h as f32)
		};
		let (x, y) = (x / w, 1. - y / h);
		let (dx, dy) = (rand(-1.0, 1.0), rand(-1.0, 1.0));

		self.uniforms.blobs_positions[self.uniforms.blobs_count as usize] = (x, y);
		self.blobs_velocities[self.uniforms.blobs_count as usize] = (dx, dy);
		self.uniforms.blobs_count += 1;
	}

	fn draw(&mut self) {
		self.uniforms.time = (miniquad_wasm_bindgen::date::now() - self.start_time) as f32;

		self.ctx.begin_default_pass(Default::default());
		self.ctx.apply_pipeline(&self.pipeline);
		self.ctx.apply_bindings(&self.bindings);
		self.ctx.apply_uniforms(UniformsSource::table(&self.uniforms));
		self.ctx.draw(0, 6, 1);
		self.ctx.end_render_pass();

		self.ctx.commit_frame();
	}
}

fn rand(from: f32, to: f32) -> f32 {
	miniquad_wasm_bindgen::date::now().fract().sin() as f32 * (to - from) + from
}

fn main() {
	let mut conf = conf::Conf::default();
	conf.platform.linux_backend = LinuxBackend::WaylandWithX11Fallback;

	miniquad_wasm_bindgen::start(conf, move || Box::new(Stage::new()));
}

// based on: https://www.shadertoy.com/view/XsS3DV
mod shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying highp vec2 uv;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        uv = in_uv;
    }"#;

	pub const FRAGMENT: &str = r#"#version 100
    precision highp float;

    varying vec2 uv;

    uniform float time;
    uniform int blobs_count;
    uniform vec2 blobs_positions[32];

    float k = 20.0;
    float field = 0.0;
    vec2 coord;

    void circle ( float r , vec3 col , vec2 offset) {
        vec2 pos = coord.xy;
        vec2 c = offset;
        float d = distance ( pos , c );
        field += ( k * r ) / ( d*d );
    }

    vec3 band ( float shade, float low, float high, vec3 col1, vec3 col2 ) {
        if ( (shade >= low) && (shade <= high) ) {
            float delta = (shade - low) / (high - low);
            vec3 colDiff = col2 - col1;
            return col1 + (delta * colDiff);
        }
        else
            return vec3(0.0,0.0,0.0);
    }

    vec3 gradient ( float shade ) {
        vec3 colour = vec3( (sin(time/2.0)*0.25)+0.25,0.0,(cos(time/2.0)*0.25)+0.25);

        vec3 col1 = vec3(0.01, 0.0, 1.0-0.01);
        vec3 col2 = vec3(1.0-0.01, 0.0, 0.01);
        vec3 col3 = vec3(0.02, 1.0-0.02, 0.02);
        vec3 col4 = vec3((0.01+0.02)/2.0, (0.01+0.02)/2.0, 1.0 - (0.01+0.02)/2.0);
        vec3 col5 = vec3(0.02, 0.02, 0.02);

        colour += band ( shade, 0.0, 0.3, colour, col1 );
        colour += band ( shade, 0.3, 0.6, col1, col2 );
        colour += band ( shade, 0.6, 0.8, col2, col3 );
        colour += band ( shade, 0.8, 0.9, col3, col4 );
        colour += band ( shade, 0.9, 1.0, col4, col5 );

        return colour;
    }

    void main() {
        coord = uv;

        for (int i = 0; i < 32; i++) {
            if (i >= blobs_count) { break; } // workaround for webgl error: Loop index cannot be compared with non-constant expression
            circle(.03 , vec3(0.7 ,0.2, 0.8), blobs_positions[i]);
        }

        float shade = min ( 1.0, max ( field/256.0, 0.0 ) );

        gl_FragColor = vec4( gradient(shade), 1.0 );
    }"#;

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec![],
			uniforms: UniformBlockLayout {
				uniforms: vec![
					UniformDesc::new("time", UniformType::Float1),
					UniformDesc::new("blobs_count", UniformType::Int1),
					UniformDesc::new("blobs_positions", UniformType::Float2).array(32),
				],
			},
		}
	}

	#[repr(C)]
	pub struct Uniforms {
		pub time: f32,
		pub blobs_count: i32,
		pub blobs_positions: [(f32, f32); 32],
	}
}
